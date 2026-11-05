pub mod routes;
pub mod handlers;
pub mod models;

use axum::
Router;

#[tokio::main]
async fn main() {
    let app: Router = routes::new_router();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .unwrap();

    println!("Server running on port 8080");

    axum::serve(listener, app)
        .await
        .unwrap();
}