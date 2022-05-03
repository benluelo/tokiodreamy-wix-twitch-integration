use std::{error::Error, net::SocketAddr, sync::Arc};

use axum::{
    extract::{
        ws::{Message, WebSocket},
        WebSocketUpgrade,
    },
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Extension, Json, Router,
};

use models::{
    wix::{NewOrder, OrderNumber},
    ClientMsg, OrderWithJson, ServerMsg,
};
use parking_lot::RwLock;
use sqlx::{postgres::PgPoolOptions, query, query_as, PgPool};
use tokio::{
    select,
    sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();
    dotenv::dotenv().unwrap();

    let ws_connection = Arc::new(RwLock::new(None));

    let pool = PgPoolOptions::new()
        .max_connections(3)
        .connect(&dotenv::var("DATABASE_URL")?)
        .await?;

    let app = Router::new()
        .route("/all_orders", get(all_orders))
        .route(
            "/new_order",
            post({
                let ws = Arc::clone(&ws_connection);
                move |db, json| new_order(ws, db, json)
            }),
        )
        .route(
            "/ws",
            get({
                let channel_to_websocket = Arc::clone(&ws_connection);
                move |db, ws_upgrade| ws_handler(channel_to_websocket, db, ws_upgrade)
            }),
        )
        .layer(Extension(pool));

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

#[tracing::instrument(skip(channel_to_websocket, db))]
async fn new_order(
    channel_to_websocket: Arc<RwLock<Option<UnboundedSender<OrderNumber>>>>,
    Extension(db): Extension<PgPool>,
    Json(new_order): Json<NewOrder>,
) -> impl IntoResponse {
    let order_number = new_order.order_number;
    tracing::info!("recieved order #{}", order_number);

    // TODO: Don't unwrap
    let twitch_username = new_order.twitch_username().unwrap();
    let json_value = serde_json::to_value(new_order)
        .expect("Object was deserialized from JSON, should not fail.");

    if let Err(why) = query_as!(
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
        tracing::error!("error inserting into the database: {}", why);
        StatusCode::INTERNAL_SERVER_ERROR
    } else {
        tracing::info!("order #{} saved successfully", order_number);

        let notifier = channel_to_websocket.read_recursive();

        if let Some(tx) = &*notifier {
            tracing::info!(
                "client is connected, attempting to notify about order #{}",
                order_number
            );
            if let Err(why) = tx.send(order_number) {
                tracing::error!("error sending across stream: {}", why);
            }
        } else {
            tracing::info!(
                "client not connected, order #{} added to queue",
                order_number
            );
        }

        StatusCode::OK
    }
}

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
        Ok(all_orders) => Json(all_orders).into_response(),
        Err(why) => {
            tracing::error!("error selecting from the database: {}", why);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

#[tracing::instrument(skip_all)]
async fn ws_handler(
    channel_to_websocket: Arc<RwLock<Option<UnboundedSender<OrderNumber>>>>,
    Extension(db): Extension<PgPool>,
    ws_upgrade: WebSocketUpgrade,
) -> impl IntoResponse {
    ws_upgrade.on_upgrade(move |socket| {
        let (sender, reciever) = unbounded_channel::<OrderNumber>();
        {
            let mut write_lock = channel_to_websocket.write();
            *write_lock = Some(sender);
        }
        let db = db.clone();
        handle_socket(db, reciever, socket /* channel_to_websocket */)
    })
}

#[tracing::instrument(skip_all)]
async fn handle_socket(
    db: PgPool,
    mut new_order_notification_receiver: UnboundedReceiver<OrderNumber>,
    mut websocket: WebSocket,
) {
    loop {
        select! {
            Some(new_order_id) = new_order_notification_receiver.recv() => {
                match query_as!(
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
                {
                    Ok(new_order) => {
                        if let Err(why) = websocket.send(Message::Text(
                            serde_json::to_string(&ServerMsg::NewOrder{new_order: new_order}).expect("serializing should not fail"),
                        )).await {
                            tracing::error!("error sending message: {}", why)
                        }
                    }
                    Err(why) => {
                        tracing::error!("error querying the database: {}", why);
                    }
                }
            },

            Some(msg) = websocket.recv() => {
                if let Ok(msg) = msg {
                match msg {
                    Message::Text(json) => match serde_json::from_str::<ClientMsg>(&json) {
                        Ok(client_msg) => match client_msg {
                            ClientMsg::BreakCompleted { order_number } => {
                                let server_msg = match query!(
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
                                        ServerMsg::BreakCompletedSuccess { order_number }
                                    }
                                    Err(why) => {
                                        tracing::error!("error deleting from the database: {}", why);
                                        ServerMsg::BreakCompletedError { order_number }
                                    }
                                };

                                let message_to_send = Message::Text(
                                    serde_json::to_string(&server_msg)
                                        .expect("serializing should not fail"),
                                );

                                if let Err(why) = websocket.send(message_to_send).await {
                                    tracing::error!("error sending message: {}", why)
                                }

                                tracing::info!(
                                    "successfully sent websocket message about order #{}",
                                    &order_number
                                );
                            }
                        },
                        Err(why) => tracing::error!("error deserializing client message: {}", why),
                    },
                    Message::Close(_) => {
                        // client disconnected
                        if let Err(why) = websocket.close().await {
                            tracing::error!("error gracefully closing websocket connection: {}", why);
                        };
                        return;
                    },
                    _ => {}
                }
                } else {
                    // client disconnected
                    if let Err(why) = websocket.close().await {
                        tracing::error!("error gracefully closing websocket connection: {}", why);
                    };
                    return;
                }
            },
            else => continue,
        }
    }
}
