use axum::{http::StatusCode, response::IntoResponse};

use crate::auth::AuthorizedUser;

#[tracing::instrument]
pub(crate) async fn get(_: AuthorizedUser) -> impl IntoResponse {
    StatusCode::OK
}
