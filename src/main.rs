use axum::{Router};
use tower_http::trace::TraceLayer;
use std::net::SocketAddr;

mod routes;

#[tokio::main]
async fn main() {
    // Initialize router
    let app = Router::new()
        .nest("/api/users", routes::user_routes::init_routes())
        .layer(TraceLayer::new_for_http()); // Use TraceLayer for logging

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running at http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
