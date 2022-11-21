mod hello_world;
mod mirror_body_string;
mod mirror_body_json;
mod path_variables;

use axum::{body::Body, routing::{get, post}, Router};
use hello_world::hello_world;
use mirror_body_string::mirror_body_string;
use mirror_body_json::mirror_body_json;
use path_variables::{path_variables, hard_coded_path};

pub fn create_routes() -> Router<Body> {
    Router::new()
        .route("/", get(hello_world))
        .route("/mirror-body-string", post(mirror_body_string))
        .route("/mirror-body-json", post(mirror_body_json))
        .route("/path-variables/:id", get(path_variables))
        .route("/path-variables/15", get(hard_coded_path))
}