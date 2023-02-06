use axum::{
    body::{boxed, Body, BoxBody},
    http::{Request, Response, StatusCode, Uri},
};
use tower::ServiceExt;
use tower_http::services::ServeDir;

pub(crate) async fn file_handler(uri: Uri) -> Result<Response<BoxBody>, StatusCode> {
    get_static_file(uri.clone()).await
}

async fn get_static_file(uri: Uri) -> Result<Response<BoxBody>, StatusCode> {
    let req = Request::builder().uri(uri).body(Body::empty()).unwrap();

    // `ServeDir` implements `tower::Service` so we can call it with `tower::ServiceExt::oneshot`
    // When run normally, the root is the workspace root
    match ServeDir::new("~/personal-projects/tokio-wix-backend/src/static")
        .oneshot(req)
        .await
    {
        Ok(res) => Ok(res.map(boxed)),
        Err(err) => {
            tracing::error!("Something went wrong: {}", err);

            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
