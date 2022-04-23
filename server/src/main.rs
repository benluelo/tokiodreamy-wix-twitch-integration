use std::{error::Error, net::SocketAddr};

use axum::{
    body::Body,
    extract::{
        ws::{Message, WebSocket},
        WebSocketUpgrade,
    },
    http::{Request, StatusCode},
    response::IntoResponse,
    routing::get,
    Extension, Json, Router,
};
use entity::order::Order;

use models::{wix::NewOrder, ClientMsg, ServerMsg};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, query, query_as, PgPool};
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();
    dotenv::dotenv().unwrap();

    let pool = PgPoolOptions::new()
        .max_connections(3)
        .connect(&dotenv::var("DATABASE_URL")?)
        .await?;

    let app = Router::new()
        .route("/new_order", get(new_order))
        .route("/ws", get(ws_handler))
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

async fn new_order(
    ws_channel: Option<Extension<UnboundedSender<Uuid>>>,
    Extension(db): Extension<PgPool>,
    Json(new_order): Json<NewOrder>,
) -> impl IntoResponse {
    let timestamp = new_order._date_created.naive_utc().clone();
    let twitch_username = new_order
        .custom_field
        .as_ref()
        .map(|cf| cf.value.clone())
        .unwrap();
    let order_id = new_order._id;
    let json_value = serde_json::to_value(new_order).unwrap();

    if let Err(why) = query_as!(
        OrderWithJson,
        r#"
        INSERT INTO public.order (
            twitch_username,
            json,
            order_id,
            date_created
        )
        VALUES ($1, $2, $3, $4)
        ON CONFLICT DO NOTHING
        "#,
        twitch_username,
        json_value,
        order_id,
        timestamp,
    )
    .execute(&db)
    .await
    {
        tracing::error!("error inserting into the database: {}", why);
        return StatusCode::OK;
    };

    if let Some(Extension(tx)) = ws_channel {
        if let Err(why) = tx.send(order_id) {
            tracing::error!("error sending across stream: {}", why);
        }
    }

    StatusCode::OK
}

async fn ws_handler(mut req: Request<Body>, ws: WebSocketUpgrade) -> impl IntoResponse {
    let (sender, reciever) = unbounded_channel::<Uuid>();
    req.extensions_mut().insert(Extension(sender));
    let db = req
        .extensions()
        .get::<Extension<PgPool>>()
        .expect("pool to be present in the extensions");
    let db = db.0.clone();

    let upgrade = ws.on_upgrade(move |socket| handle_socket(db, reciever, socket));

    upgrade
}

async fn handle_socket(db: PgPool, mut reciever: UnboundedReceiver<Uuid>, mut socket: WebSocket) {
    while let Some(msg) = socket.recv().await {
        if let Ok(msg) = msg {
            match msg {
                Message::Text(json) => match serde_json::from_str::<ClientMsg>(&json) {
                    Ok(client_msg) => match client_msg {
                        ClientMsg::BreakCompleted { order_id } => {
                            let message_to_send = match query!(
                                r#"
                                DELETE FROM public.order
                                WHERE order_id = $1
                                "#,
                                order_id,
                            )
                            .execute(&db)
                            .await
                            {
                                Ok(_) => Message::Text(
                                    serde_json::to_string(&ServerMsg::BreakCompletedSuccess {
                                        order_id,
                                    })
                                    .expect("serializing should not fail"),
                                ),
                                Err(why) => {
                                    tracing::error!("error deleting from the database: {}", why);
                                    continue;
                                }
                            };

                            if let Err(why) = socket.send(message_to_send).await {
                                tracing::error!("error sending message: {}", why)
                            }
                        }
                    },
                    Err(why) => tracing::error!("error deserializing client message: {}", why),
                },
                Message::Binary(_) => todo!(),
                Message::Ping(_) => todo!(),
                Message::Pong(_) => todo!(),
                Message::Close(_) => todo!(),
            }

            let mut new_orders = vec![];

            while let Some(new_order_id) = reciever.recv().await {
                match query_as!(
                    Order,
                    r#"
                    SELECT twitch_username, order_id, date_created FROM public.order
                    WHERE order_id = $1
                    "#,
                    new_order_id,
                )
                .fetch_one(&db)
                .await
                {
                    Ok(order) => new_orders.push(order),
                    Err(why) => {
                        tracing::error!("error inserting into the database: {}", why);
                    }
                };
            }
        } else {
            // client disconnected
            if let Err(why) = socket.close().await {
                tracing::error!("error gracefully closing websocket connection: {}", why);
            };
            return;
        }
    }
}
