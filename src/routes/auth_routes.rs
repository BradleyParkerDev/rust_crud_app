use axum::{Router, routing::post, Json};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct AuthResponse {
    token: String,
}

// Initialize auth-related routes
pub fn init_routes() -> Router {
    Router::new()
        .route("/login", post(login))
        .route("/logout", post(logout))
}

// Handlers
async fn login(Json(payload): Json<LoginRequest>) -> Json<AuthResponse> {
    println!("Logging in user: {}", payload.username);
    Json(AuthResponse {
        token: "example-token".to_string(),
    })
}

async fn logout() -> &'static str {
    println!("Logging out user");
    "Logged out"
}
