use sqlx::PgPool;

use crate::{
    errors::EzyTutorError,
    models::course::{Course, CreateCourse, UpdateCourse},
};

pub async fn get_courses_for_tutor_db(
    pool: &PgPool,
    tutor_id: i32,
) -> Result<Vec<Course>, EzyTutorError> {
    let courses: Vec<Course> = sqlx::query_as!(
        Course,
        "select * from ezy_course_c6 where tutor_id = $1",
        tutor_id // $1에 해당하는 듯
    )
    .fetch_all(pool)
    .await?;

    match courses.len() {
        0 => Err(EzyTutorError::NotFound(
            "Courses not found for tutor".into(),
        )),
        _ => Ok(courses),
    }
}

pub async fn get_course_details_db(
    pool: &PgPool,
    tutor_id: i32,
    course_id: i32,
) -> Result<Course, EzyTutorError> {
    let course_row = sqlx::query_as!(
        Course,
        "select * from ezy_course_c6 where tutor_id = $1 and course_id = $2",
        tutor_id,
        course_id
    )
    .fetch_optional(pool)
    .await?;

    if let Some(course) = course_row {
        Ok(course)
    } else {
        Err(EzyTutorError::NotFound("Course id not found!".into()))
    }
}

pub async fn post_new_course_db(
    pool: &PgPool,
    course: CreateCourse,
) -> Result<Course, EzyTutorError> {
    let course_insert = sqlx::query_as!(
        Course,
        "insert into ezy_course_c6 (
            tutor_id, course_name, course_description, course_duration, course_format, course_structure, course_price, course_language, course_level
        ) values ($1,$2,$3,$4,$5,$6,$7,$8,$9) returning *",
           course.tutor_id,course.course_name,course.course_description,course.course_duration,course.course_format,course.course_structure,course.course_price,course.course_language,course.course_level
    )
    .fetch_one(pool)
    .await?;
    Ok(course_insert)
}

pub async fn delete_course_db(
    pool: &PgPool,
    tutor_id: i32,
    course_id: i32,
) -> Result<Course, EzyTutorError> {
    let course_delete = sqlx::query_as!(
        Course,
        "delete from ezy_course_c6 where tutor_id = $1 and course_id = $2 returning *",
        tutor_id,
        course_id
    )
    .fetch_one(pool)
    .await?;
    Ok(course_delete)
}

pub async fn update_course_details_db(
    pool: &PgPool,
    tutor_id: i32,
    course_id: i32,
    update_course: UpdateCourse,
) -> Result<Course, EzyTutorError> {
    let course = sqlx::query_as!(
        Course,
        "select * from ezy_course_c6 where tutor_id = $1 and course_id = $2",
        tutor_id,
        course_id
    )
    .fetch_one(pool)
    .await?;
    let name = if let Some(_) = update_course.course_name {
        update_course.course_name
    } else {
        Some(course.course_name)
    };
    let description = if let Some(_) = update_course.course_description {
        update_course.course_description
    } else {
        course.course_description
    };
    let format = if let Some(_) = update_course.course_format {
        update_course.course_format
    } else {
        course.course_format
    };
    let structure = if let Some(_) = update_course.course_structure {
        update_course.course_structure
    } else {
        course.course_structure
    };
    let duration = if let Some(_) = update_course.course_duration {
        update_course.course_duration
    } else {
        course.course_duration
    };
    let price = if let Some(_) = update_course.course_price {
        update_course.course_price
    } else {
        course.course_price
    };
    let language = if let Some(_) = update_course.course_language {
        update_course.course_language
    } else {
        course.course_language
    };
    let level = if let Some(_) = update_course.course_level {
        update_course.course_level
    } else {
        course.course_level
    };

    let course_row = sqlx::query_as!(
        Course,
        "update ezy_course_c6 set course_name = $1, course_description = $2, course_format = $3, course_structure = $4, course_duration = $5, course_price = $6, course_language = $7, course_level = $8 where tutor_id = $9 and course_id = $10 returning *",
        name, description, format, structure, duration, price, language, level, tutor_id, course_id
    ).fetch_one(pool).await;
    if let Ok(c) = course_row {
        Ok(c)
    } else {
        Err(EzyTutorError::NotFound("Course id not found".into()))
    }
}
