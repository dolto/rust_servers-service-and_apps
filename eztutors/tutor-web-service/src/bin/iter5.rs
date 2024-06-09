use std::{env, io, sync::Mutex};

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use errors::EzyTutorError;
use routes::{course_routes, general_routes, tutor_routes};
use sqlx::PgPool;
use state::AppState;

#[path = "../iter5/dbaccess/mod.rs"]
mod dbaccess;
#[path = "../iter5/errors.rs"]
mod errors;
#[path = "../iter5/handlers/mod.rs"]
mod handlers;
#[path = "../iter5/models/mod.rs"]
mod models;
#[path = "../iter5/routes.rs"]
mod routes;
#[path = "../iter5/state.rs"]
mod state;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let db_pool = PgPool::connect(&database_url).await.unwrap();

    let shared_data = web::Data::new(AppState {
        health_check_response: "난 좋아, 이 페이지에 방문한 사람 수: ".to_owned(),
        visit_count: Mutex::new(0),
        db: db_pool,
    });

    let app = move || {
        App::new()
            .app_data(shared_data.clone())
            .app_data(web::JsonConfig::default().error_handler(|err, _| {
                EzyTutorError::InvalidInput(format!(
                    "Json 데이터가 잘못되어있습니다! 에러내용: {}",
                    err.to_string()
                ))
                .into()
            }))
            .configure(general_routes)
            .configure(course_routes)
            .configure(tutor_routes)
    };
    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}
