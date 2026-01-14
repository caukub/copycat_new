use axum::{
    body::Body,
    extract::Path,
    http::header::CONTENT_TYPE,
    response::{IntoResponse, Redirect, Response},
};
use tokio_util::io::ReaderStream;
use tracing::{debug, error};

#[tracing::instrument]
pub async fn raw_handler(Path(id): Path<String>) -> Response {
    let file = match tokio::fs::File::open(&format!("data/{}", id)).await {
        Ok(file) => file,
        Err(err) => {
            debug!("Failed to open '{}': {}", id, err);
            return Redirect::to("/").into_response();
        }
    };

    let stream = ReaderStream::new(file);
    let body = Body::from_stream(stream);

    Response::builder()
        .header(CONTENT_TYPE, "text/plain; charset=utf-8")
        .header("X-Content-Type-Options", "nosniff")
        .body(body)
        .unwrap_or_else(|err| {
            error!("Failed to build the Response: {}", err);
            Redirect::to("/").into_response()
        })
}
