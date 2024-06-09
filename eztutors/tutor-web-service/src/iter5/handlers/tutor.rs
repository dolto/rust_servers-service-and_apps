use actix_web::{web, HttpResponse};

use crate::{
    dbaccess::tutor::{
        delete_tutor_db, get_all_tutor_db, get_tutor_details_db, post_new_tutor_db,
        update_tutor_details_db,
    },
    errors::EzyTutorError,
    models::tutor::{NewTutor, UpdateTutor},
    state::AppState,
};

pub async fn post_new_tutor(
    app_state: web::Data<AppState>,
    new_tutor: web::Json<NewTutor>,
) -> Result<HttpResponse, EzyTutorError> {
    let tutor = post_new_tutor_db(&app_state.db, new_tutor).await?;
    Ok(HttpResponse::Ok().json(tutor))
}

pub async fn get_all_tutor(app_state: web::Data<AppState>) -> Result<HttpResponse, EzyTutorError> {
    let tutors = get_all_tutor_db(&app_state.db).await?;
    Ok(HttpResponse::Ok().json(tutors))
}

pub async fn get_tutor_details(
    app_state: web::Data<AppState>,
    paths: web::Path<i32>,
) -> Result<HttpResponse, EzyTutorError> {
    let tutor_id = paths.into_inner();
    let tutor = get_tutor_details_db(&app_state.db, tutor_id).await?;
    Ok(HttpResponse::Ok().json(tutor))
}

pub async fn update_tutor_details(
    app_state: web::Data<AppState>,
    paths: web::Path<i32>,
    base_tutor: web::Json<UpdateTutor>,
) -> Result<HttpResponse, EzyTutorError> {
    let tutor_id = paths.into_inner();
    let tutor = update_tutor_details_db(&app_state.db, tutor_id, base_tutor.into()).await?;
    Ok(HttpResponse::Ok().json(tutor))
}

pub async fn delete_tutor(
    app_state: web::Data<AppState>,
    paths: web::Path<i32>,
) -> Result<HttpResponse, EzyTutorError> {
    let tutor_id = paths.into_inner();
    let tutor = delete_tutor_db(&app_state.db, tutor_id).await?;
    Ok(HttpResponse::Ok().json(tutor))
}
