use actix_web::web;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

// use crate::errors::EzyTutorError;

#[derive(Serialize, Debug, Clone, FromRow)]
pub struct Course {
    pub course_id: i32,
    pub tutor_id: i32,
    pub course_name: String,
    pub posted_time: NaiveDateTime,
    pub course_description: Option<String>,
    pub course_format: Option<String>,
    pub course_structure: Option<String>,
    pub course_duration: Option<String>,
    pub course_price: Option<i32>,
    pub course_language: Option<String>,
    pub course_level: Option<String>,
}

// 프론트에서 새로운 강의를 만들 때 들어갈 정보만 담은 구조체
#[derive(Deserialize, Debug, Clone)]
pub struct CreateCourse {
    pub tutor_id: i32,
    pub course_name: String,
    pub course_description: Option<String>,
    pub course_format: Option<String>,
    pub course_structure: Option<String>,
    pub course_duration: Option<String>,
    pub course_price: Option<i32>,
    pub course_language: Option<String>,
    pub course_level: Option<String>,
}
impl From<web::Json<CreateCourse>> for CreateCourse {
    fn from(value: web::Json<CreateCourse>) -> Self {
        CreateCourse {
            tutor_id: value.tutor_id,
            course_name: value.course_name.clone(),
            course_description: value.course_description.clone(),
            course_format: value.course_format.clone(),
            course_structure: value.course_structure.clone(),
            course_duration: value.course_duration.clone(),
            course_price: value.course_price,
            course_language: value.course_language.clone(),
            course_level: value.course_level.clone(),
        }
    }
}
// impl TryFrom<web::Json<Create>> for CreateCourse {
//     type Error = EzyTutorError;
//     fn try_from(value: web::Json<CreateCourse>) -> Result<Self, Self::Error> {
//         Ok(value.into())
//     }
// }

// 만든 강의를 수정할 때 굳이 강사 id가 필요 없다
#[derive(Debug, Deserialize, Clone)]
pub struct UpdateCourse {
    pub course_name: Option<String>,
    pub course_description: Option<String>,
    pub course_format: Option<String>,
    pub course_structure: Option<String>,
    pub course_duration: Option<String>,
    pub course_price: Option<i32>,
    pub course_language: Option<String>,
    pub course_level: Option<String>,
}
impl From<web::Json<UpdateCourse>> for UpdateCourse {
    fn from(value: web::Json<UpdateCourse>) -> Self {
        UpdateCourse {
            course_name: value.course_name.clone(),
            course_description: value.course_description.clone(),
            course_format: value.course_format.clone(),
            course_structure: value.course_structure.clone(),
            course_duration: value.course_duration.clone(),
            course_price: value.course_price,
            course_language: value.course_language.clone(),
            course_level: value.course_level.clone(),
        }
    }
}
