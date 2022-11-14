use std::fmt::format;
use crate::{AppState, TokenClaims};
use actix_web::{
    get, post,
    web::{Data, Json, ReqData},
    HttpResponse, Responder,
};
use actix_web_httpauth::extractors::basic::BasicAuth;
use argonautica::{Hasher, Verifier};
use chrono::NaiveDateTime;
use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use sqlx::{self, FromRow};

#[derive(Deserialize)]
struct CreateUserBody {
    username: String,
    password: String,
}

#[derive(Serialize, FromRow)]
struct UserNoPassword {
    id: i32,
    username: String,
}

#[post("/user")]
pub async fn create_user(state: Data<AppState>, body: Json<CreateUserBody>) -> impl Responder {
    let user: CreateUserBody = body.into_inner();

    let hash_secret = std::env::var("HASH_SECRET").expect("HASH_SECRET must be set!");
    let mut hasher = Hasher::default();
    let hash = hasher
        .with_password(user.password)
        .with_secret_key(hash_secret)
        .hash()
        .unwrap();

    match sqlx::query_as::<_, UserNoPassword>(
        "INSERT INTO users (username, password)\
        VALUE ($1, $2)\
        RETURNING id, username"
    )
        .bind(user.username)
        .bind(hash)
        .fetch_one(&state.db)
        .await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(error) => HttpResponse::InternalServerError().json(format!("{:?}", error))
    }
}

#[get("/auth")]
pub async fn basic_auth(state: Data<AppState>) -> impl Responder {}

#[post("/article")]
pub async fn create_article(state: Data<AppState>) -> impl Responder {}