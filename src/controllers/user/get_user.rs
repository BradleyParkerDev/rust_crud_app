
async fn get_user() -> impl Responder {
    HttpResponse::Ok().body("User data found!")
}

