use axum::Json;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestUser {
    username: String,
    password: String,
}

pub async fn validate_with_serde(Json(user): Json<RequestUser>) {
    dbg!(user);
}