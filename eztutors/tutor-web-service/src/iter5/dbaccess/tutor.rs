use actix_web::{body::MessageBody, web};
use sqlx::PgPool;

use crate::{
    errors::EzyTutorError,
    models::tutor::{NewTutor, Tutor, UpdateTutor},
};

pub async fn post_new_tutor_db(
    pool: &PgPool,
    new_tutor: web::Json<NewTutor>,
) -> Result<Tutor, EzyTutorError> {
    let tutor = sqlx::query_as!(
        Tutor,
        "insert into ezy_tutor_c6 (tutor_name, tutor_pic_url, tutor_profile) values($1, $2, $3) returning *",
        new_tutor.tutor_name, new_tutor.tutor_pic_url, new_tutor.tutor_profile
    )
    .fetch_one(pool)
    .await.map_err(|msg| EzyTutorError::DBError(msg.to_string()));
    tutor
}

pub async fn get_all_tutor_db(pool: &PgPool) -> Result<Vec<Tutor>, EzyTutorError> {
    sqlx::query_as!(Tutor, "select * from ezy_tutor_c6")
        .fetch_all(pool)
        .await
        .map_err(|_| {
            EzyTutorError::NotFound("There is no tutors... where is every body?".to_owned())
        })
}

pub async fn get_tutor_details_db(pool: &PgPool, tutor_id: i32) -> Result<Tutor, EzyTutorError> {
    sqlx::query_as!(
        Tutor,
        "select * from ezy_tutor_c6 where tutor_id = $1",
        tutor_id
    )
    .fetch_one(pool)
    .await
    .map_err(|_| EzyTutorError::NotFound("There is no tutor...".to_owned()))
}
pub async fn update_tutor_details_db(
    pool: &PgPool,
    tutor_id: i32,
    base_tutor: UpdateTutor,
) -> Result<Tutor, EzyTutorError> {
    let tutor = sqlx::query_as!(
        Tutor,
        "select * from ezy_tutor_c6 where tutor_id = $1",
        tutor_id
    )
    .fetch_one(pool)
    .await?;

    let name = if base_tutor.tutor_name.is_some() {
        base_tutor.tutor_name
    } else {
        Some(tutor.tutor_name)
    };
    let pic_url = if base_tutor.tutor_pic_url.is_some() {
        base_tutor.tutor_pic_url
    } else {
        Some(tutor.tutor_pic_url)
    };
    let profile = if base_tutor.tutor_profile.is_some() {
        base_tutor.tutor_profile
    } else {
        Some(tutor.tutor_profile)
    };

    sqlx::query_as!(
        Tutor,
        "update ezy_tutor_c6 set tutor_name = $1, tutor_pic_url = $2, tutor_profile = $3 where tutor_id = $4 returning * ",
        name, pic_url, profile, tutor_id
    )
    .fetch_one(pool)
    .await.map_err(|e| EzyTutorError::DBError(e.to_string()))
}
pub async fn delete_tutor_db(pool: &PgPool, tutor_id: i32) -> Result<Tutor, EzyTutorError> {
    sqlx::query_as!(
        Tutor,
        "delete from ezy_tutor_c6 where tutor_id = $1 returning * ",
        tutor_id
    )
    .fetch_one(pool)
    .await
    .map_err(|_| {
        EzyTutorError::NotFound("There is no tutors... what do you want to delete?".to_owned())
    })
}
