use actix_web::web;

use super::handlers::{handle_register, show_register_form};

pub fn app_config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("")
            .service(actix_files::Files::new("/static", "./static").show_files_listing())
            .service(web::resource("/").route(web::get().to(show_register_form)))
            .service(web::resource("/register").route(web::post().to(handle_register))),
    );
}
