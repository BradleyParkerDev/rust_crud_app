use axum::{Router, routing::get, response::Html};

// Initialize web-related routes
pub fn init_routes() -> Router {
    Router::new()
        .route("/", get(home))
        .route("/about", get(about))
}

// Handlers
async fn home() -> Html<&'static str> {
    Html("<h1>Welcome to the Home Page</h1>")
}

async fn about() -> Html<&'static str> {
    Html("<h1>About Us</h1>")
}
