use sqlx::PgPool;

use crate::models::Course;

pub async fn get_courses_for_tutor_db(pool: &PgPool, tutor_id: i32) -> Vec<Course> {
    let course_rows = sqlx::query!(
        "select * from ezy_course_c4 where tutor_id = $1",
        tutor_id // $1에 해당하는 듯
    )
    .fetch_all(pool)
    .await
    .unwrap();

    course_rows
        .iter()
        .map(|course_row| Course {
            course_id: course_row.course_id,
            tutor_id: course_row.tutor_id,
            course_name: course_row.course_name.clone(),
            posted_time: Some(chrono::NaiveDateTime::from(course_row.posted_time.unwrap())),
        })
        .collect()
}

pub async fn get_course_details_db(pool: &PgPool, tutor_id: i32, course_id: i32) -> Course {
    let course_row = sqlx::query!(
        "select * from ezy_course_c4 where tutor_id = $1 and course_id = $2",
        tutor_id,
        course_id
    )
    .fetch_one(pool)
    .await
    .unwrap();

    Course {
        course_id: course_row.course_id,
        course_name: course_row.course_name.clone(),
        tutor_id: course_row.tutor_id,
        posted_time: Some(chrono::NaiveDateTime::from(course_row.posted_time.unwrap())),
    }
}

pub async fn post_new_course_db(pool: &PgPool, course: Course) -> Course {
    let course_insert = sqlx::query!(
        "insert into ezy_course_c4 (
            course_id, tutor_id, course_name
        ) values ($1,$2,$3) returning *",
        course.course_id,
        course.tutor_id,
        course.course_name
    )
    .fetch_one(pool)
    .await
    .unwrap();

    Course {
        course_id: course_insert.course_id,
        course_name: course_insert.course_name.clone(),
        tutor_id: course_insert.tutor_id,
        posted_time: Some(chrono::NaiveDateTime::from(
            course_insert.posted_time.unwrap(),
        )),
    }
}
