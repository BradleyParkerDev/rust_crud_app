// use actix_web::{web, HttpResponse};
// use crate::controllers::user::{get_user, register_user, update_user, delete_user};

// pub fn init_user_routes(cfg: &mut web::ServiceConfig) {
//     cfg.service(
//         web::scope("/user")
//             .route("/get", web::get().to(get_user))           // GET /api/user/get
//             .route("/register", web::post().to(register_user)) // POST /api/user/register
//             .route("/update", web::put().to(update_user))      // PUT /api/user/update
//             .route("/delete", web::delete().to(delete_user)),  // DELETE /api/user/delete
//     );
// }
