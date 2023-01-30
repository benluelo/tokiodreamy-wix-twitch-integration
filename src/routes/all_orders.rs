use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use sqlx::{query_as, PgPool};

use crate::{
    auth::AuthorizedUser,
    models::{wix::OrderNumber, OrderWithJson, OrderWithOrder},
};

#[tracing::instrument(skip_all)]
pub(crate) async fn get(_: AuthorizedUser, Extension(db): Extension<PgPool>) -> impl IntoResponse {
    all_orders(db)
        .await
        .map(Json)
        .map_err(|()| StatusCode::INTERNAL_SERVER_ERROR)
        .into_response()
}

async fn all_orders(db: PgPool) -> Result<Vec<OrderWithOrder>, ()> {
    query_as!(
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
    .map(|all_orders| {
        all_orders
            .into_iter()
            .map(Into::<OrderWithOrder>::into)
            .collect::<Vec<_>>()
    })
    .map_err(|why| {
        tracing::error!("error selecting from the database: {}", why);
        ()
    })
}
