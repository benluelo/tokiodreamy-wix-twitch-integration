use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use sqlx::{query, PgPool};
use tokio::sync::watch;

use crate::models::{wix::OrderNumber, Breaks};

#[tracing::instrument(skip(sender, db))]
pub(crate) async fn post(
    Path(order_number): Path<OrderNumber>,
    State(sender): State<Arc<watch::Sender<Breaks>>>,
    State(db): State<PgPool>,
) -> StatusCode {
    sender.send_modify(|breaks| breaks.remove_by_id(order_number));

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
            StatusCode::OK
        }
        Err(why) => {
            tracing::error!("error deleting from the database: {}", &why);

            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
