use std::{io::Error, sync::Mutex};

use actix_web::{web, App, HttpServer};
use routes::{course_routes, general_routes};
use state::AppState;

#[path = "../handlers.rs"]
mod handlers;
#[path = "../models.rs"]
mod models;
#[path = "../routes.rs"]
mod routes;
#[path = "../state.rs"]
mod state;

#[actix_rt::main]
async fn main() -> Result<(), Error> {
    let shared_data = web::Data::new(AppState {
        helth_check_response: "I'm good. You've already asked me".to_string(),
        visit_count: Mutex::new(0),
        courses: Mutex::new(vec![]),
    });
    let app = move || {
        App::new()
            .app_data(shared_data.clone())
            .configure(general_routes)
            .configure(course_routes)
    };

    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}
