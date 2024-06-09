use std::{convert::Infallible, fmt::Display};

use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum EzyTutorError {
    DBError(String),
    ActixError(String),
    NotFound(String),
}
impl std::error::Error for EzyTutorError {}
impl EzyTutorError {
    fn error_response(&self) -> String {
        match self {
            EzyTutorError::DBError(msg) => {
                println!(
                    "데이터 베이스에서 에러가 발생했습니다! 에러 메세지: {}",
                    msg
                );
                "Database error".into()
            }
            EzyTutorError::ActixError(msg) => {
                println!("서버에서 에러가 발생했습니다! 에러 메세지: {}", msg);
                "Internal server error".into()
            }
            EzyTutorError::NotFound(msg) => {
                println!(
                    "찾을수 없음(Not Found)에러가 발생했습니다! 에러 메세지: {}",
                    msg
                );
                msg.into()
            }
        }
    }
}
impl ResponseError for EzyTutorError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            EzyTutorError::DBError(_) | EzyTutorError::ActixError(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            EzyTutorError::NotFound(_) => StatusCode::NOT_FOUND,
        }
    }
    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code()).json(MyErrorResponse {
            error_message: self.error_response(),
        })
    }
}
impl Display for EzyTutorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
impl From<actix_web::Error> for EzyTutorError {
    fn from(value: actix_web::error::Error) -> Self {
        EzyTutorError::ActixError(value.to_string())
    }
}
impl From<sqlx::Error> for EzyTutorError {
    fn from(value: sqlx::error::Error) -> Self {
        EzyTutorError::DBError(value.to_string())
    }
}
impl From<Infallible> for EzyTutorError {
    fn from(value: Infallible) -> Self {
        EzyTutorError::NotFound(value.to_string())
    }
}

#[derive(Debug, Serialize)]
pub struct MyErrorResponse {
    error_message: String,
}
