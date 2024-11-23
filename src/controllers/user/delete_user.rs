
async fn delete_user() -> impl Responder {
    HttpResponse::Ok().body("User successfully deleted!")
}

