use std::{error::Error, net::SocketAddr, path::PathBuf, sync::Arc};

use axum::{
    extract::Path,
    http::StatusCode,
    response::{
        sse::{Event, KeepAlive},
        IntoResponse, Sse,
    },
    routing::{get, post},
    Extension, Json, Router,
};
use axum_extra::routing::SpaRouter;
use clap::Parser;
use futures::{
    channel::mpsc::{unbounded, UnboundedSender},
    Stream, StreamExt,
};
use models::{
    wix::{NewOrder, OrderNumber},
    OrderWithJson, OrderWithOrder, SseEvent,
};
use parking_lot::RwLock;
use sqlx::{postgres::PgPoolOptions, query, query_as, PgPool};

use crate::auth::AuthorizedUser;

#[derive(Debug, clap::Parser)]
struct Args {
    /// Path to the dotenv file containing the required environment variables.
    #[clap(long)]
    dotenv_file_path: PathBuf,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    tracing_subscriber::fmt::init();
    dotenv::from_path(args.dotenv_file_path).unwrap();

    let pool = PgPoolOptions::new()
        .max_connections(3)
        .connect(&dotenv::var("DATABASE_URL")?)
        .await?;

    // `SpaRouter` is the easiest way to serve assets at a nested route like `/assets`
    // let app = Router::new()
    //     .route("/foo", get(|| async { "Hi from /foo" }))
    //     .merge(axum_extra::routing::SpaRouter::new("/assets", "."))
    //     .layer(TraceLayer::new_for_http());

    let app = Router::new()
        .route("/all_orders", get(all_orders))
        .route("/order_completed/:order_number", get(order_completed))
        .route("/sse", get(sse_handler))
        .route("/new_order", post(new_order))
        .merge(SpaRouter::new("/downloads", "./downloads"))
        .layer(Extension(pool))
        .layer(Extension(Arc::new(RwLock::<
            Option<UnboundedSender<OrderNumber>>,
        >::new(None))));

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

async fn sse_handler(
    _: AuthorizedUser,
    Extension(channel_to_websocket): Extension<Arc<RwLock<Option<UnboundedSender<OrderNumber>>>>>,
    Extension(db): Extension<PgPool>,
    // TODO: Better error type
) -> Sse<impl Stream<Item = Result<Event, String>>> {
    let (new_order_notification_sender, new_order_notification_receiver) =
        unbounded::<OrderNumber>();
    {
        let mut write_lock = channel_to_websocket.write();
        *write_lock = Some(new_order_notification_sender);
    }

    let event_stream = new_order_notification_receiver.then(move |new_order_id| {
        let db = db.clone();
        async move {
            query_as!(
                OrderWithJson,
                r#"
                SELECT
                    twitch_username,
                    order_id as "order_id: OrderNumber",
                    json
                FROM public.order
                WHERE order_id = $1::INT
                "#,
                new_order_id as OrderNumber,
            )
            .fetch_one(&db.clone())
            .await
            .map(Into::<OrderWithOrder>::into)
            .map(SseEvent::NewOrder)
            .map(|ok| Event::default().json_data(ok).unwrap())
            .map_err(|err| {
                tracing::error!("error querying the database: {}", err);
                err.to_string()
            })
        }
    });

    Sse::new(event_stream).keep_alive(KeepAlive::default())
}

#[tracing::instrument(skip_all)]
async fn new_order(
    Extension(channel_to_websocket): Extension<Arc<RwLock<Option<UnboundedSender<OrderNumber>>>>>,
    Extension(db): Extension<PgPool>,
    Json(new_order): Json<NewOrder>,
) -> impl IntoResponse {
    let order_number = new_order.order_number;
    tracing::info!("recieved order #{}", order_number);

    // TODO: Don't unwrap
    let twitch_username = new_order.twitch_username().unwrap();
    let json_value = serde_json::to_value(new_order)
        .expect("Object was deserialized from JSON, should not fail.");

    match query_as!(
        OrderWithJson,
        r#"
        INSERT INTO public.order (
            twitch_username,
            json,
            order_id
        )
        VALUES ($1, $2, $3)
        ON CONFLICT DO NOTHING
        "#,
        twitch_username,
        json_value,
        order_number as OrderNumber,
    )
    .execute(&db)
    .await
    {
        Err(why) => {
            tracing::error!("error inserting into the database: {}", why);
            StatusCode::INTERNAL_SERVER_ERROR
        }
        Ok(ok) => {
            if ok.rows_affected() == 0 {
                tracing::info!("duplicate order received (#{})", order_number);
            } else {
                tracing::info!("order #{} saved successfully", order_number);

                let notifier = channel_to_websocket.read_recursive();

                if let Some(tx) = &*notifier {
                    tracing::info!(
                        "client is connected, attempting to notify about order #{}",
                        order_number
                    );
                    if let Err(why) = tx.unbounded_send(order_number) {
                        tracing::error!("error sending across stream: {}", why);
                    }
                } else {
                    tracing::info!(
                        "client not connected, order #{} added to queue",
                        order_number
                    );
                }
            }

            StatusCode::OK
        }
    }
}

#[tracing::instrument(skip_all)]
async fn all_orders(_: AuthorizedUser, Extension(db): Extension<PgPool>) -> impl IntoResponse {
    match query_as!(
        OrderWithJson,
        r#"
        SELECT
            twitch_username,
            order_id as "order_id: OrderNumber",
            json
        FROM public.order
        "#,
    )
    .fetch_all(&db)
    .await
    {
        Ok(all_orders) => Json(
            all_orders
                .into_iter()
                .map(Into::<OrderWithOrder>::into)
                .collect::<Vec<_>>(),
        )
        .into_response(),
        Err(why) => {
            tracing::error!("error selecting from the database: {}", why);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

#[tracing::instrument(skip_all)]
async fn order_completed(
    Path(order_number): Path<OrderNumber>,
    Extension(db): Extension<PgPool>,
) -> impl IntoResponse {
    match query!(
        r#"
        DELETE FROM public.order
        WHERE order_id = $1::INT
        "#,
        order_number as OrderNumber,
    )
    .execute(&db)
    .await
    {
        Ok(_) => {
            tracing::info!("successfully deleted order #{}", &order_number);
            StatusCode::OK.into_response()
        }
        Err(why) => {
            tracing::error!("error deleting from the database: {}", &why);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(format!("error deleting from the database: {}", why)),
            )
                .into_response()
        }
    }
}

mod auth {
    use axum::{
        async_trait,
        extract::{FromRequest, RequestParts},
        http::{header::AUTHORIZATION, StatusCode},
    };
    use sqlx::{query_as, PgPool};

    pub struct AuthorizedUser {
        who: String,
    }

    #[async_trait]
    impl<B> FromRequest<B> for AuthorizedUser
    where
        B: Send, // required by `async_trait`
    {
        type Rejection = StatusCode;

        async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
            let db = req.extensions().get::<PgPool>().unwrap();
            let auth_header: &str = req
                .headers()
                .get(AUTHORIZATION)
                .ok_or(StatusCode::UNAUTHORIZED)?
                .to_str()
                .map_err(|_| StatusCode::UNAUTHORIZED)?;
            let decoded = base64::decode(auth_header)
                .map_err(|_| StatusCode::UNAUTHORIZED)?
                .split(|&c| c == ':' as u8)
                .map(|b| String::from_utf8(b.into()))
                .collect::<Result<Vec<_>, _>>()
                .map_err(|_| StatusCode::UNAUTHORIZED)?;

            let (username, key) = match &*decoded {
                [username, key] => (username, key),
                _ => return Err(StatusCode::UNAUTHORIZED),
            };

            struct Exists {
                exists: bool,
            }

            match query_as!(
                Exists,
                r#"
                SELECT
                EXISTS(
                    SELECT 1
                    FROM public.authentication_keys
                    WHERE
                        username = $1
                    AND
                        key = $2
                )
                AS "exists!"
                "#,
                username,
                key
            )
            .fetch_one(db)
            .await
            {
                Ok(Exists { exists: true }) => Ok(AuthorizedUser {
                    who: username.clone(),
                }),
                Ok(Exists { exists: false }) => Err(StatusCode::UNAUTHORIZED),
                Err(why) => {
                    tracing::error!("error selecting from the database: {}", why);
                    Err(StatusCode::INTERNAL_SERVER_ERROR)
                }
            }
        }
    }
}
