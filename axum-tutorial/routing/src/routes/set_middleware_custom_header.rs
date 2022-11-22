use axum::http::{Request, StatusCode};
use axum::middleware::Next;
use axum::response::Response;
use crate::routes::middleware_custom_header::HeaderMessage;

pub async fn set_middleware_custom_header<B>(mut request: Request<B>, next: Next<B>) -> Result<Response, StatusCode> {
    let headers = request.headers();
    let message = headers
        .get("message")
        .ok_or_else(|| StatusCode::BAD_REQUEST)?;
    let message = message
        .to_str()
        .map_err(|_| StatusCode::BAD_REQUEST)?
        .to_owned();
    request.extensions_mut().insert(HeaderMessage(message));
    Ok(next.run(request).await)
}