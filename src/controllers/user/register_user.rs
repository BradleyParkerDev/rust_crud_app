
async fn register_user() -> impl Responder {
    HttpResponse::Created().body("User successfully registered!")
}

