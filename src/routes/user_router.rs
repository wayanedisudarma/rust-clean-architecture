use axum::Router;
use axum::routing::get;
use crate::handlers::user_handler::get_users;

pub fn user_routes() -> Router {
    Router::new()
    .route("/", get(get_users))
}