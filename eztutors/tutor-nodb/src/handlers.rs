use actix_web::{web, HttpResponse};
use chrono::Utc;

use crate::{models::Course, state::AppState};

pub async fn health_check_handler(app_state: web::Data<AppState>) -> HttpResponse {
    let health_check_response = &app_state.helth_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap();
    let response = format!("{} {} times", health_check_response, visit_count);
    *visit_count += 1;
    HttpResponse::Ok().json(&response)
}

pub async fn new_course(
    new_course: web::Json<Course>,
    app_state: web::Data<AppState>,
) -> HttpResponse {
    println!("new course");
    let course_count_for_user = app_state
        .courses
        .lock()
        .unwrap()
        .clone()
        .into_iter()
        .filter(|c| c.tutor_id == new_course.tutor_id)
        .count(); // 등록하려고 하는 튜터의 코스 수
    let new_course = Course {
        tutor_id: new_course.tutor_id,
        course_id: Some(course_count_for_user + 1),
        course_name: new_course.course_name.clone(),
        posted_time: Some(Utc::now().naive_utc()),
    };
    app_state.courses.lock().unwrap().push(new_course);
    HttpResponse::Ok().json("Added course")
}

pub async fn get_courses_for_tutor(
    app_state: web::Data<AppState>,
    user_id: web::Path<usize>,
) -> HttpResponse {
    let user_id = user_id.clone();
    let filtered = app_state
        .courses
        .lock()
        .unwrap()
        .clone()
        .into_iter()
        .filter(|c| c.tutor_id == user_id)
        .collect::<Vec<Course>>();

    if filtered.len() > 0 {
        HttpResponse::Ok().json(filtered)
    } else {
        HttpResponse::Ok().json("No courses found for tutor".to_string())
    }
}

pub async fn get_courses_detail(
    app_state: web::Data<AppState>,
    params: web::Path<(usize, usize)>,
) -> HttpResponse {
    let user_id = params.0;
    let course_id = params.1;
    let course = app_state
        .courses
        .lock()
        .unwrap()
        .clone()
        .into_iter()
        .filter(|c| c.tutor_id == user_id && c.course_id.unwrap() == course_id)
        .collect::<Vec<Course>>();
    let course = course.get(0);

    match course {
        Some(course) => HttpResponse::Ok().json(course),
        None => HttpResponse::Ok().json("No courses found for tutor of course id".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use actix_web::http::StatusCode;

    use super::*;

    #[actix_rt::test]
    async fn post_course_test() {
        let test_str = "Hello, this is test course".to_string();
        // 가상의 요청 데이터를 만들고
        let course = web::Json(Course {
            tutor_id: 1,
            course_id: None,
            posted_time: None,
            course_name: test_str,
        });
        // 가상의 기존 (상태)데이터를 생성
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            helth_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            courses: Mutex::new(vec![]),
        });

        // 요청이 들어왔을 시 수행되는 함수 new_course를 수행
        let resp = new_course(course, app_state).await;

        // 요청이 정상 처리가 되었는지 확인
        assert_eq!(resp.status(), StatusCode::OK);
    }
    #[actix_rt::test]
    async fn get_all_courses_success() {
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            helth_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            courses: Mutex::new(vec![]),
        });
        let tutor_id: web::Path<usize> = web::Path::from(1);
        let resp = get_courses_for_tutor(app_state, tutor_id).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }
    #[actix_rt::test]
    async fn get_detail_courses_success() {
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            helth_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            courses: Mutex::new(vec![]),
        });
        let params: web::Path<(usize, usize)> = web::Path::from((1, 1));
        let resp = get_courses_detail(app_state, params).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }
}
