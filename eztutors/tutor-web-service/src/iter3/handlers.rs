use actix_web::{web, HttpResponse};

use crate::{
    db_access::{get_course_details_db, get_courses_for_tutor_db, post_new_course_db},
    models::Course,
    state::AppState,
};

pub async fn health_check_handler(app_state: web::Data<AppState>) -> HttpResponse {
    let health_check_response = &app_state.health_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap();
    let response = format!("{} {}", health_check_response, visit_count);
    *visit_count += 1;
    HttpResponse::Ok().json(response)
}

pub async fn get_courses_for_tutor(
    app_state: web::Data<AppState>,
    params: web::Path<(i32,)>,
) -> HttpResponse {
    // let tutor_id = params.0; // 아마 숫자가 아닌경우, 다른 형식의 res를 반환하는 방법이 있나봄
    let tutor_id = i32::try_from(params.0).unwrap_or(1);
    let courses = get_courses_for_tutor_db(&app_state.db, tutor_id).await;
    HttpResponse::Ok().json(courses)
}

pub async fn get_course_details(
    app_state: web::Data<AppState>,
    params: web::Path<(i32, i32)>,
) -> HttpResponse {
    let tutor_id = i32::try_from(params.0).unwrap_or(1);
    let course_id = i32::try_from(params.1).unwrap_or(1);
    // 아마 이 구간에서 실패할 경우 400번대 에러를 반환하게끔 하는 로직이 있는거같음
    let course = get_course_details_db(&app_state.db, tutor_id, course_id).await;
    HttpResponse::Ok().json(course)
}

pub async fn post_new_course(
    app_state: web::Data<AppState>,
    new_course: web::Json<Course>,
) -> HttpResponse {
    let course = post_new_course_db(&app_state.db, new_course.into()).await;
    HttpResponse::Ok().json(course)
}

#[cfg(test)]
mod tests {
    use std::{env, sync::Mutex};

    use actix_web::http::StatusCode;
    use dotenv::dotenv;
    use sqlx::PgPool;

    use super::*;

    #[actix_rt::test]
    async fn get_all_courses_success() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let pool: PgPool = PgPool::connect(&database_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: pool,
        });
        let tutor_id: web::Path<(i32,)> = web::Path::from((1,));
        let resp = get_courses_for_tutor(app_state, tutor_id).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_course_detail_test() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let pool: PgPool = PgPool::connect(&database_url).await.unwrap();

        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: pool,
        });
        let params: web::Path<(i32, i32)> = web::Path::from((1, 2));
        let resp = get_course_details(app_state, params).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }
    #[actix_rt::test]
    async fn post_course_cuccess() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let pool: PgPool = PgPool::connect(&database_url).await.unwrap();

        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: pool,
        });
        let _ = sqlx::query!("delete from ezy_course_c4 where course_id=3")
            .fetch_all(&app_state.db)
            .await;
        let new_course_msg = Course {
            course_id: 3,
            tutor_id: 1,
            course_name: "This is teh next course".into(),
            posted_time: None,
        };
        let course_param = web::Json(new_course_msg);
        let resp = post_new_course(app_state, course_param).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }
}
