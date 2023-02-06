use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use sqlx::{query, PgPool};

use crate::{
    auth::AuthorizedUser,
    models::{
        wix::{NewOrder, OrderNumber},
        OrderWithOrder,
    },
};

#[tracing::instrument(skip_all)]
pub(crate) async fn get(_: AuthorizedUser, State(db): State<PgPool>) -> impl IntoResponse {
    all_orders(db)
        .await
        .map(Json)
        .map_err(|()| StatusCode::INTERNAL_SERVER_ERROR)
        .into_response()
}

async fn all_orders(db: PgPool) -> Result<Vec<OrderWithOrder>, ()> {
    query!(
        r#"
        SELECT
            twitch_username,
            order_id as "order_id: OrderNumber",
            json as "order: sqlx::types::Json<NewOrder>"
        FROM public.order
        "#,
    )
    .fetch_all(&db)
    .await
    .map(|all_orders| {
        all_orders
            .into_iter()
            .map(|record| OrderWithOrder {
                twitch_username: record.twitch_username,
                order_id: record.order_id,
                order: record.order.0,
            })
            .collect::<Vec<_>>()
    })
    .map_err(|why| {
        tracing::error!("error selecting from the database: {}", why);
    })
}
