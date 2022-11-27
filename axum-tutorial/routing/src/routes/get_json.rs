use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Data {
    message: String,
    count: i32,
    username: String,
}

pub async fn get_json() -> Json<Data> {
    let data = Data { message: "I'm in data~!".to_owned(), count: 432, username: "Hello".to_owned() };
    Json(data)
}