use std::{error::Error, net::SocketAddr, sync::Arc};

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

use futures::{
    channel::mpsc::{unbounded, UnboundedSender},
    Stream, StreamExt,
};
use models::{
    wix::{NewOrder, OrderNumber},
    OrderWithJson, OrderWithOrder,
};
use parking_lot::RwLock;
use sqlx::{
    postgres::{PgPoolOptions, PgQueryResult},
    query, query_as, PgPool,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();
    dotenv::dotenv().unwrap();

    let pool = PgPoolOptions::new()
        .max_connections(3)
        .connect(&dotenv::var("DATABASE_URL")?)
        .await?;

    let app = Router::new()
        .route("/all_orders", get(all_orders))
        .route("/order_completed/:order_number", get(order_completed))
        .route("/sse", get(sse_handler))
        .route("/new_order", post(new_order))
        .layer(Extension(pool))
        .layer(Extension(Arc::new(RwLock::<
            Option<UnboundedSender<OrderNumber>>,
        >::new(None))));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn sse_handler(
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

    let strm = new_order_notification_receiver.then(move |new_order_id| {
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
            .map(|ok| Event::default().json_data(ok).unwrap())
            .map_err(|err| {
                tracing::error!("error querying the database: {}", err);
                err.to_string()
            })
        }
    });

    Sse::new(strm).keep_alive(KeepAlive::default())
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
async fn all_orders(Extension(db): Extension<PgPool>) -> impl IntoResponse {
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
