use actix_web::{web, HttpResponse};

use crate::{
    dbaccess::course::{
        delete_course_db, get_course_details_db, get_courses_for_tutor_db, post_new_course_db,
        update_course_details_db,
    },
    errors::EzyTutorError,
    models::course::{CreateCourse, UpdateCourse},
    state::AppState,
};

pub async fn get_courses_for_tutor(
    app_state: web::Data<AppState>,
    params: web::Path<(i32,)>,
) -> Result<HttpResponse, EzyTutorError> {
    let tutor_id = i32::try_from(params.0)?;
    let courses = get_courses_for_tutor_db(&app_state.db, tutor_id)
        .await
        .map(|c| HttpResponse::Ok().json(c));
    courses
}

pub async fn get_course_details(
    app_state: web::Data<AppState>,
    params: web::Path<(i32, i32)>,
) -> Result<HttpResponse, EzyTutorError> {
    let tutor_id = i32::try_from(params.0)?;
    let course_id = i32::try_from(params.1)?;
    // 아마 이 구간에서 실패할 경우 400번대 에러를 반환하게끔 하는 로직이 있는거같음
    let course = get_course_details_db(&app_state.db, tutor_id, course_id)
        .await
        .map(|c| HttpResponse::Ok().json(c));
    course
}

pub async fn post_new_course(
    app_state: web::Data<AppState>,
    new_course: web::Json<CreateCourse>,
) -> Result<HttpResponse, EzyTutorError> {
    let course = post_new_course_db(&app_state.db, new_course.into())
        .await
        .map(|c| HttpResponse::Ok().json(c));
    course
}

pub async fn delete_course(
    app_state: web::Data<AppState>,
    path: web::Path<(i32, i32)>,
) -> Result<HttpResponse, EzyTutorError> {
    let (tutor_id, course_id) = path.into_inner();
    delete_course_db(&app_state.db, tutor_id, course_id)
        .await
        .map(|c| HttpResponse::Ok().json(c))
}

pub async fn update_course_details(
    app_state: web::Data<AppState>,
    update_course: web::Json<UpdateCourse>,
    path: web::Path<(i32, i32)>,
) -> Result<HttpResponse, EzyTutorError> {
    let (tutor_id, course_id) = path.into_inner();
    update_course_details_db(&app_state.db, tutor_id, course_id, update_course.into())
        .await
        .map(|c| HttpResponse::Ok().json(c))
}

#[cfg(test)]
mod tests {
    use core::panic;
    use std::{env, sync::Mutex};

    use actix_web::{http::StatusCode, ResponseError};
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
        let resp = get_courses_for_tutor(app_state, tutor_id).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_course_detail_success_test() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let pool: PgPool = PgPool::connect(&database_url).await.unwrap();

        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: pool,
        });
        let params: web::Path<(i32, i32)> = web::Path::from((1, 2));
        let resp = get_course_details(app_state, params).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
    #[actix_rt::test]
    async fn get_course_detail_failure_test() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let pool: PgPool = PgPool::connect(&database_url).await.unwrap();

        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: pool,
        });
        let params: web::Path<(i32, i32)> = web::Path::from((1, 99));
        let resp = get_course_details(app_state, params).await;

        match resp {
            Ok(_) => panic!("Update Success! Why?"),
            Err(err) => assert_eq!(err.status_code(), StatusCode::NOT_FOUND),
        };
    }
    #[actix_rt::test]
    async fn post_and_delete_course_cuccess() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let pool: PgPool = PgPool::connect(&database_url).await.unwrap();

        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: pool,
        });
        let new_course_msg = CreateCourse {
            tutor_id: 1,
            course_name: "This is teh next course".into(),
            course_description: Some("this is a test course".into()),
            course_format: None,
            course_level: Some("Beginner".into()),
            course_price: None,
            course_duration: Some("1h 30m".into()),
            course_language: Some("Korean".into()),
            course_structure: None,
        };
        let course_param = web::Json(new_course_msg);
        let resp = post_new_course(app_state.clone(), course_param)
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);

        // let params = web::Path::from((1, 5));
        // let resp = delete_course(app_state, params).await.unwrap();
        // assert_eq!(resp.status(), StatusCode::OK);
    }
    #[actix_rt::test]
    async fn update_course_cuccess() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let pool: PgPool = PgPool::connect(&database_url).await.unwrap();

        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: pool,
        });
        let update_course_msg = UpdateCourse {
            course_name: Some("This is teh next course".into()),
            course_description: Some("this is a update2 course".into()),
            course_format: None,
            course_level: Some("Beginner".into()),
            course_price: Some(1),
            course_duration: Some("3h 30m".into()),
            course_language: Some("Korean".into()),
            course_structure: None,
        };
        let course_param = web::Json(update_course_msg);
        let params: web::Path<(i32, i32)> = web::Path::from((1, 2));
        let resp = update_course_details(app_state, course_param, params)
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
}
