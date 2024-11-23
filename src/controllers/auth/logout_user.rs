
async fn logout_user() -> impl Responder {
    HttpResponse::Ok().body("User successfully logged out!")
}