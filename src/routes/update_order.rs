use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::{query, PgPool};
use tokio::sync::watch;
use ts_rs::TS;

use crate::{
    auth::AuthorizedUser,
    models::{wix::OrderNumber, Breaks},
};

#[tracing::instrument(skip(sender, db))]
pub(crate) async fn post(
    _: AuthorizedUser,
    Path(order_number): Path<OrderNumber>,
    State(sender): State<Arc<watch::Sender<Breaks>>>,
    State(db): State<PgPool>,
    Json(update): Json<OrderUpdate>,
) -> impl IntoResponse {
    match update {
        OrderUpdate::Name(name) => {
            sender.send_modify(|breaks| {
                breaks
                    .get_mut_by_id(order_number)
                    .map(|brk| brk.twitch_username.replace(name.clone()));
            });

            // TODO(benluelo): name length <= 64

            match query!(
                r#"
                    UPDATE public.order
                    SET twitch_username = $1
                    WHERE order_id = $2
                "#,
                name,
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

                    StatusCode::INTERNAL_SERVER_ERROR.into_response()
                }
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export, export_to = "frontend/src/generated/")]
pub(crate) enum OrderUpdate {
    Name(String),
}
