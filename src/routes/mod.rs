pub mod user_router;

use axum::Router;
use crate::routes::user_router::user_routes;

pub fn new_router() -> Router {
    let router = Router::new()
        .nest("/api/v1/users", user_routes());
    router
}