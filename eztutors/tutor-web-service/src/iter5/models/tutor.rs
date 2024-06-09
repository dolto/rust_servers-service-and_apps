use actix_web::web;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Tutor {
    pub tutor_id: i32,
    pub tutor_name: String,
    pub tutor_pic_url: String,
    pub tutor_profile: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct NewTutor {
    pub tutor_name: String,
    pub tutor_pic_url: String,
    pub tutor_profile: String,
}
impl From<web::Json<NewTutor>> for NewTutor {
    fn from(value: web::Json<NewTutor>) -> Self {
        NewTutor {
            tutor_name: value.tutor_name.clone(),
            tutor_pic_url: value.tutor_pic_url.clone(),
            tutor_profile: value.tutor_profile.clone(),
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct UpdateTutor {
    pub tutor_name: Option<String>,
    pub tutor_pic_url: Option<String>,
    pub tutor_profile: Option<String>,
}
impl From<web::Json<UpdateTutor>> for UpdateTutor {
    fn from(value: web::Json<UpdateTutor>) -> Self {
        UpdateTutor {
            tutor_name: value.tutor_name.clone(),
            tutor_pic_url: value.tutor_pic_url.clone(),
            tutor_profile: value.tutor_profile.clone(),
        }
    }
}
