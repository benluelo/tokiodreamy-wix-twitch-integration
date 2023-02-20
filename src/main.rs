use std::{error::Error, net::SocketAddr, path::PathBuf, sync::Arc};

use axum::{
    extract::FromRef,
    http::{
        header::{AUTHORIZATION, CONTENT_TYPE},
        Method, StatusCode,
    },
    routing::{get, get_service, post},
    Router,
};
use clap::Parser;
use sqlx::{postgres::PgPoolOptions, PgPool};
use tokio::sync::watch;
use tower_http::{
    cors::{AllowOrigin, CorsLayer},
    services::ServeDir,
};

use crate::{
    models::Breaks,
    routes::{all_orders, login, new_order, order_completed, sse, update_order},
};

mod auth;
mod models;
mod routes;

const FRONT_PUBLIC: &str = "./frontend/build";

#[derive(Debug, clap::Parser)]
struct Args {
    /// Path to the dotenv file containing the required environment variables.
    #[clap(long, short = 'e')]
    dotenv_file_path: PathBuf,

    #[clap(long, short = 'p')]
    port: u16,
}

#[derive(Clone, FromRef)]
pub struct AppState {
    pub pool: PgPool,
    pub breaks_sender: Arc<watch::Sender<Breaks>>,
    pub breaks_reciever: watch::Receiver<Breaks>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();

    let args = Args::parse();
    dotenv::from_path(args.dotenv_file_path).unwrap();

    let (breaks_sender, breaks_reciever) =
        tokio::sync::watch::channel::<Breaks>(Breaks::initialize());
    let breaks_sender = Arc::new(breaks_sender);

    let pool = PgPoolOptions::new()
        // elephant sql free tier limits to a maximum of 5 connections. Use 1 for pgadmin, 1 for
        // psql, 3 for this server
        .max_connections(3)
        .connect(&dotenv::var("DATABASE_URL")?)
        .await?;

    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([AUTHORIZATION, CONTENT_TYPE])
        // allow requests from any origin
        // .allow_origin(Any);
        // .allow_origin("http://127.0.0.1:3000".parse::<HeaderValue>().unwrap());
        .allow_origin(AllowOrigin::mirror_request());

    let frontend_static = Router::<AppState>::new()
        .fallback_service(
            get_service(ServeDir::new(FRONT_PUBLIC)).handle_error(
                |error: std::io::Error| async move {
                    tracing::error!("Unhandled internal error: {}", error);

                    StatusCode::INTERNAL_SERVER_ERROR
                },
            ), // .handle_error(handle_error)
        )
        .layer(tower_http::trace::TraceLayer::new_for_http());

    let backend_router = Router::new()
        .route("/all_orders", get(all_orders::get))
        .route(
            "/order_completed/:order_number",
            post(order_completed::post),
        )
        .route("/sse", get(sse::get))
        .route("/login", get(login::get))
        .route("/new_order", post(new_order::post))
        .route("/update_order/:order_number", post(update_order::post))
        .layer(cors);

    let app = Router::new()
        .merge(frontend_static)
        .merge(backend_router)
        .with_state(AppState {
            pool,
            breaks_sender,
            breaks_reciever,
        });

    // // configure certificate and private key used by https
    // let config = RustlsConfig::from_pem_file(
    //     PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    //         .join("self_signed_certs")
    //         .join("cert.pem"),
    //     PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    //         .join("self_signed_certs")
    //         .join("key.pem"),
    // )
    // .await
    // .unwrap();

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([0, 0, 0, 0], args.port));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
