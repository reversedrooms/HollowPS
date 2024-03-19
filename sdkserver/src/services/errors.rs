use axum::http::{StatusCode, Uri};
use axum::response::IntoResponse;

pub async fn not_found(uri: Uri) -> impl IntoResponse {
    tracing::warn!("unhandled http request: {uri}");
    StatusCode::NOT_FOUND
}
