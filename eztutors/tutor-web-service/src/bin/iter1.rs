use std::{env, io::Result};

use chrono::NaiveDateTime;
use dotenv::dotenv;
use sqlx::PgPool;

#[derive(Debug)]
pub struct Course {
    pub course_id: i32,
    pub tutor_id: i32,
    pub course_name: String,
    pub posted_time: Option<NaiveDateTime>,
}

#[actix_rt::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");

    // db 커넥션 풀을 만든다. 이를통해 여러 데이터베이스 커넥션을 관리할 수 있다
    let db_pool = PgPool::connect(&database_url).await.unwrap();

    // 사용할 쿼리를 사용한다. .env의 DATABASE_URL설정을 하면 컴파일 지점에서 자동으로 유효성 체크를 한다
    let course_rows = sqlx::query!(
        r#"select course_id, tutor_id, course_name, posted_time from ezy_course_c4 where course_id = $1"#,
        1
    ).fetch_all(&db_pool).await.unwrap();

    let mut courses_list = vec![];
    for course_row in course_rows {
        courses_list.push(Course {
            course_id: course_row.course_id,
            tutor_id: course_row.tutor_id,
            course_name: course_row.course_name,
            posted_time: Some(chrono::NaiveDateTime::from(course_row.posted_time.unwrap())),
        })
    }
    println!("Courses = {:?}", courses_list);
    Ok(())
}
