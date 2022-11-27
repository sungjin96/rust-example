mod hello_world;
mod mirror_body_string;
mod mirror_body_json;
mod path_variables;
mod query_params;
mod mirror_user_agent;
mod mirror_custom_header;
mod middleware_message;
mod middleware_custom_header;
mod set_middleware_custom_header;
mod always_errors;
mod returns_201;

use axum::{body::Body, routing::{get, post}, Router, Extension, middleware};
use axum::http::Method;
use tower_http::cors::{CorsLayer, Any};
use hello_world::hello_world;
use mirror_body_string::mirror_body_string;
use mirror_body_json::mirror_body_json;
use path_variables::{path_variables, hard_coded_path};
use query_params::query_params;
use mirror_user_agent::mirror_user_agent;
use mirror_custom_header::mirror_custom_header;
use middleware_message::middleware_message;
use middleware_custom_header::middleware_custom_header;
use set_middleware_custom_header::set_middleware_custom_header;
use always_errors::always_errors;
use returns_201::returns_201;

#[derive(Clone)]
pub struct SharedData {
    pub message: String,
}

pub fn create_routes() -> Router<Body> {
    let cors = CorsLayer::new()
        .allow_methods([Method::POST, Method::GET, Method::DELETE, Method::PUT])
        .allow_origin(Any);
    let shared_data = SharedData { message: "Hello from shared data".to_owned() };

    Router::new()
        .route("/middleware-custom-header", get(middleware_custom_header))
        .route_layer(middleware::from_fn(set_middleware_custom_header))
        .route("/", get(hello_world))
        .route("/mirror-body-string", post(mirror_body_string))
        .route("/mirror-body-json", post(mirror_body_json))
        .route("/path-variables/:id", get(path_variables))
        .route("/path-variables/15", get(hard_coded_path))
        .route("/query-params", get(query_params))
        .route("/mirror-user-agent", get(mirror_user_agent))
        .route("/mirror-custom-header", get(mirror_custom_header))
        .route("/middleware-message", get(middleware_message))
        .layer(Extension(shared_data))
        .layer(cors)
        .route("/always-errors", get(always_errors))
        .route("/returns-201", post(returns_201))

}