use std::{error::Error, net::SocketAddr, path::PathBuf, sync::Arc};

use axum::{
    http::{
        header::{AUTHORIZATION, CONTENT_TYPE},
        Method,
    },
    routing::{get, post},
    Extension, Router,
};
use clap::Parser;
use sqlx::postgres::PgPoolOptions;
use tower_http::cors::{AllowOrigin, CorsLayer};

use crate::{
    models::Breaks,
    routes::{all_orders, login, new_order, order_completed, sse},
};

mod auth;
mod models;
mod routes;

#[derive(Debug, clap::Parser)]
struct Args {
    /// Path to the dotenv file containing the required environment variables.
    #[clap(long)]
    dotenv_file_path: PathBuf,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let (sender, reciever) = tokio::sync::watch::channel::<Breaks>(Breaks::initialize());
    let sender = Arc::new(sender);

    tracing_subscriber::fmt::init();
    dotenv::from_path(args.dotenv_file_path).unwrap();

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

    let app = Router::new()
        .route("/all_orders", get(all_orders::get))
        .route(
            "/order_completed/:order_number",
            post(order_completed::post),
        )
        .route("/sse", get(sse::get))
        .route("/login", get(login::get))
        .route("/new_order", post(new_order::post))
        .layer(cors)
        .layer(Extension(pool))
        .layer(Extension(sender))
        .layer(Extension(reciever));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
