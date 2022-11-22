use axum::Extension;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct HeaderMessage(pub String);

pub async fn middleware_custom_header(Extension(message): Extension<HeaderMessage>) -> String {
    message.0
}
