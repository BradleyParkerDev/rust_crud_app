async fn login_user() -> impl Responder {
    HttpResponse::Ok().body("User successfully logged in!")
}