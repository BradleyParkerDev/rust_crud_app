// use actix_web::{web, HttpResponse};
// use crate::controllers::Controllers;

// async fn login_user(data: web::Data<Controllers>) -> HttpResponse {
//     data.auth.login_user(); // Call the AuthController's login_user method
//     HttpResponse::Ok().json("Logged in successfully")
// }

// async fn logout_user(data: web::Data<Controllers>) -> HttpResponse {
//     data.auth.logout_user(); // Call the AuthController's logout_user method
//     HttpResponse::Ok().json("Logged out successfully")
// }

// pub fn init_auth_routes(cfg: &mut web::ServiceConfig, controllers: web::Data<Controllers>) {
//     cfg.service(
//         web::scope("/auth")
//             .app_data(controllers.clone()) // Pass Controllers to scope
//             .route("/login", web::post().to(login_user))   // POST /api/auth/login
//             .route("/logout", web::post().to(logout_user)), // POST /api/auth/logout
//     );
// }
