use std::sync::Arc;

use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use sqlx::{query_as, PgPool};
use tokio::sync::watch;

use crate::models::{
    wix::{NewOrder, OrderNumber},
    Breaks, OrderWithJson, OrderWithOrder,
};

#[tracing::instrument(skip_all)]
pub(crate) async fn post(
    Extension(sender): Extension<Arc<watch::Sender<Breaks>>>,
    Extension(db): Extension<PgPool>,
    Json(new_order): Json<NewOrder>,
) -> impl IntoResponse {
    let order_number = new_order.order_number;
    tracing::info!("recieved order #{}", order_number);

    // TODO: Don't unwrap
    let twitch_username = new_order.twitch_username().unwrap();
    let json_value = serde_json::to_value(new_order)
        .expect("Object was deserialized from JSON, should not fail");

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
        &twitch_username,
        &json_value,
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

                sender.send_modify(|breaks| {
                    breaks.new_order(OrderWithOrder::from(OrderWithJson {
                        twitch_username,
                        order_id: order_number,
                        json: json_value,
                    }))
                });
            }

            StatusCode::OK
        }
    }
}
