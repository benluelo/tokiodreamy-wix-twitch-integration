use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use sqlx::{query, PgPool};
use tokio::sync::watch;

use crate::models::{
    wix::{NewOrder, OrderNumber},
    Breaks, OrderWithOrder,
};

#[tracing::instrument(skip_all)]
pub(crate) async fn post(
    State(sender): State<Arc<watch::Sender<Breaks>>>,
    State(db): State<PgPool>,
    Json(new_order): Json<NewOrder>,
) -> impl IntoResponse {
    let order_number = new_order.order_number;

    tracing::info!("recieved order #{}", order_number);

    let twitch_username = new_order.twitch_username().ok();
    let json_value = serde_json::to_value(&new_order)
        .expect("Object was deserialized from JSON, should not fail");

    match query!(
        r#"
        INSERT INTO public.order (
            twitch_username,
            json,
            order_id
        )
        VALUES ($1, $2, $3)
        ON CONFLICT DO NOTHING
        "#,
        twitch_username.as_ref(),
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
                    breaks.new_order(OrderWithOrder {
                        twitch_username,
                        order_id: order_number,
                        order: new_order,
                    })
                });
            }

            StatusCode::OK
        }
    }
}
