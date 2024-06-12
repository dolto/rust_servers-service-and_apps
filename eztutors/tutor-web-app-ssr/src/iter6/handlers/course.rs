use actix_web::{web, HttpResponse};
use awc::Client;
use serde_json::json;

use crate::models::{NewCourse, NewCourseResponse, UpdateCourse, UpdateCourseResponse};

pub async fn handle_insert_course(
    // tmpl: web::Data<tera::Tera>,
    // app_state: web::Data<AppState>,
    path: web::Path<i32>,
    params: web::Json<NewCourse>,
) -> Result<HttpResponse, actix_web::error::Error> {
    let tutor_id = path.into_inner();
    let p = &params;
    let new_course = json!({
        "tutor_id": tutor_id,
        "course_name": p.course_name,
        "course_description": p.course_desc,
        "course_format": p.course_format,
        "course_structure": p.course_structure,
        "course_duration": p.course_duration,
        "course_price": p.course_price,
        "course_language": p.course_language,
        "course_level": p.course_level
    });
    let client = Client::default();
    let res = client
        .post("http://localhost:3000/courses/")
        .send_json(&new_course)
        .await
        .unwrap()
        .body()
        .await?;
    println!("Finished call: {:?}", res);

    let course_response: NewCourseResponse = serde_json::from_str(std::str::from_utf8(&res)?)?;
    Ok(HttpResponse::Ok().json(course_response))
}
pub async fn handle_update_course(
    path: web::Path<(i32, i32)>,
    params: web::Json<UpdateCourse>,
) -> Result<HttpResponse, actix_web::error::Error> {
    let (tutor_id, course_id) = path.into_inner();
    let p = &params;
    let update_course = json!({
        "course_name": p.course_name,
        "course_description": p.course_desc,
        "course_format": p.course_format,
        "course_duration": p.course_duration,
        "course_structure": p.course_structure,
        "course_price": p.course_price,
        "course_language": p.course_language,
        "course_level": p.course_level
    });
    let client = Client::default();
    let update_url = format!("http://localhost:3000/courses/{}/{}", tutor_id, course_id);
    let res = client
        .put(update_url)
        .send_json(&update_course)
        .await
        .unwrap()
        .body()
        .await?;
    let course_response: UpdateCourseResponse = serde_json::from_str(std::str::from_utf8(&res)?)?;

    Ok(HttpResponse::Ok().json(course_response))
}
pub async fn handle_delete_course(
    path: web::Path<(i32, i32)>,
) -> Result<HttpResponse, actix_web::error::Error> {
    let (tutor_id, course_id) = path.into_inner();
    let client = Client::default();
    let delete_url = format!("http://localhost:3000/courses/{}/{}", tutor_id, course_id);
    let _ = client
        .delete(delete_url)
        .send()
        .await
        .unwrap()
        .body()
        .await?;

    Ok(HttpResponse::Ok().body("Course deleted"))
}
