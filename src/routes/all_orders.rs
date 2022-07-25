use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use sqlx::{query_as, PgPool};

use crate::{
    auth::AuthorizedUser,
    models::{wix::OrderNumber, OrderWithJson, OrderWithOrder},
};

#[tracing::instrument(skip_all)]
pub(crate) async fn get(_: AuthorizedUser, Extension(db): Extension<PgPool>) -> impl IntoResponse {
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
