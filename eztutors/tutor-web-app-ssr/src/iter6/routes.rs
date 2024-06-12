use actix_web::web;

use super::handlers::{auth::*, course::*};

pub fn app_config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("")
            .service(actix_files::Files::new("/static", "./static").show_files_listing())
            .service(web::resource("/").route(web::get().to(show_register_form)))
            .service(web::resource("/signinhome").route(web::get().to(show_signin_form)))
            .service(web::resource("/signin").route(web::post().to(handle_signin)))
            .service(web::resource("/register").route(web::post().to(handle_register))),
    );
}

pub fn course_config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/courses")
            .service(web::resource("/new/{tutor_id}").route(web::post().to(handle_insert_course)))
            .service(
                web::resource("/{tutor_id}/{course_id}").route(web::put().to(handle_update_course)),
            )
            .service(
                web::resource("/delete/{tutor_id}/{course_id}")
                    .route(web::delete().to(handle_delete_course)),
            ),
    );
}
