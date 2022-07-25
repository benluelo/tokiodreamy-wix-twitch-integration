use std::sync::Arc;

use axum::{extract::Path, http::StatusCode, response::IntoResponse, Extension, Json};
use sqlx::{query, PgPool};
use tokio::sync::watch;

use crate::models::{wix::OrderNumber, Breaks};

#[tracing::instrument(skip_all)]
pub(crate) async fn post(
    Path(order_number): Path<OrderNumber>,
    Extension(sender): Extension<Arc<watch::Sender<Breaks>>>,
    Extension(db): Extension<PgPool>,
) -> impl IntoResponse {
    sender.send_modify(|breaks| breaks.remove(order_number));

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
