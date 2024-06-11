use std::fmt;

use actix_web::{http::StatusCode, HttpResponse};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum EzyTutorError {
    DbError(String),
    ActixError(String),
    NotFound(String),
    TeraError(String),
}

#[derive(Debug, Serialize)]
pub struct MyErrorResponse {
    error_message: String,
}

impl std::error::Error for EzyTutorError {}
impl fmt::Display for EzyTutorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl EzyTutorError {
    fn error_response(&self) -> String {
        match self {
            EzyTutorError::DbError(msg) => {
                println!("데이터 베이스 에러가 들어왔습니다: {:?}", msg);
                "Database error".into()
            }
            EzyTutorError::ActixError(msg) => {
                println!("서버 에러가 들어왔스빈다: {:?}", msg);
                "Database error".into()
            }
            EzyTutorError::TeraError(msg) => {
                println!("탬플릿 에러가 들어왔습니다: {:?}", msg);
                "Database error".into()
            }
            EzyTutorError::NotFound(msg) => {
                println!("Not Found 에러가 들어왔습니다: {:?}", msg);
                "Database error".into()
            }
        }
    }
}

impl actix_web::error::ResponseError for EzyTutorError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            EzyTutorError::DbError(_)
            | EzyTutorError::ActixError(_)
            | EzyTutorError::TeraError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            EzyTutorError::NotFound(_) => StatusCode::NOT_FOUND,
        }
    }
    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code()).json(MyErrorResponse {
            error_message: self.error_response(),
        })
    }
}

impl From<actix_web::error::Error> for EzyTutorError {
    fn from(value: actix_web::error::Error) -> Self {
        EzyTutorError::ActixError(value.to_string())
    }
}

impl From<sqlx::Error> for EzyTutorError {
    fn from(value: sqlx::Error) -> Self {
        EzyTutorError::DbError(value.to_string())
    }
}
