use actix_web::web;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TutorRegisterForm {
    pub username: String,
    pub password: String,
    pub confirmation: String,
    pub imageurl: String,
    pub profile: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TutorResponse {
    pub tutor_id: i32,
    pub tutor_name: String,
    pub tutor_pic_url: String,
    pub tutor_profile: String,
}

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct User {
    pub username: String,
    pub tutor_id: Option<i32>,
    pub user_password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TutorSigninForm {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Debug)]
pub struct NewCourse {
    pub course_name: String,
    pub course_desc: String,
    pub course_format: String,
    pub course_duration: String,
    pub course_structure: Option<String>,
    pub course_price: Option<i32>,
    pub course_language: Option<String>,
    pub course_level: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewCourseResponse {
    pub course_id: Option<i32>,
    pub tutor_id: i32,
    pub course_name: String,
    pub course_desc: String,
    pub course_format: String,
    pub course_structure: Option<String>,
    pub course_duration: String,
    pub course_price: Option<i32>,
    pub course_language: Option<String>,
    pub course_level: Option<String>,
    pub posted_time: String,
}

impl From<web::Json<NewCourseResponse>> for NewCourseResponse {
    fn from(value: web::Json<NewCourseResponse>) -> Self {
        NewCourseResponse {
            tutor_id: value.tutor_id,
            course_id: value.course_id,
            course_name: value.course_name.clone(),
            course_desc: value.course_desc.clone(),
            course_format: value.course_format.clone(),
            course_structure: value.course_structure.clone(),
            course_duration: value.course_duration.clone(),
            course_price: value.course_price,
            course_language: value.course_language.clone(),
            course_level: value.course_level.clone(),
            posted_time: value.posted_time.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateCourse {
    pub course_name: Option<String>,
    pub course_desc: Option<String>,
    pub course_format: Option<String>,
    pub course_structure: Option<String>,
    pub course_duration: Option<String>,
    pub course_price: Option<i32>,
    pub course_language: Option<String>,
    pub course_level: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateCourseResponse {
    pub course_id: i32,
    pub tutor_id: i32,
    pub course_name: String,
    pub course_desc: String,
    pub course_format: String,
    pub course_structure: String,
    pub course_duration: String,
    pub course_price: i32,
    pub course_language: String,
    pub course_level: String,
    pub posted_time: String,
}
impl From<web::Json<UpdateCourseResponse>> for UpdateCourseResponse {
    fn from(value: web::Json<UpdateCourseResponse>) -> Self {
        UpdateCourseResponse {
            tutor_id: value.tutor_id,
            course_id: value.course_id,
            course_name: value.course_name.clone(),
            course_desc: value.course_desc.clone(),
            course_format: value.course_format.clone(),
            course_structure: value.course_structure.clone(),
            course_duration: value.course_duration.clone(),
            course_price: value.course_price,
            course_language: value.course_language.clone(),
            course_level: value.course_level.clone(),
            posted_time: value.posted_time.clone(),
        }
    }
}
