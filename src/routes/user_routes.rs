use axum::{
    Router,
    routing::{get, post, put, delete},
    Json,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: u32,
    name: String,
    email: String,
}

// Initialize user-related routes
pub fn init_routes() -> Router {
    Router::new()
        .route("/", get(list_users))
        .route("/", post(create_user))
        .route("/:id", get(get_user))
        .route("/:id", put(update_user))
        .route("/:id", delete(delete_user))
}

// Handlers
async fn list_users() -> Json<Vec<User>> {
    Json(vec![
        User { id: 1, name: "Alice".to_string(), email: "alice@example.com".to_string() },
        User { id: 2, name: "Bob".to_string(), email: "bob@example.com".to_string() },
    ])
}

async fn get_user() -> Json<User> {
    Json(User { id: 1, name: "Alice".to_string(), email: "alice@example.com".to_string() })
}

async fn create_user(Json(payload): Json<User>) -> &'static str {
    println!("Creating user: {:?}", payload);
    "User created"
}

async fn update_user(Json(payload): Json<User>) -> &'static str {
    println!("Updating user: {:?}", payload);
    "User updated"
}

async fn delete_user() -> &'static str {
    println!("Deleting user");
    "User deleted"
}
