use std::{fs::File, io::Read};

use actix_web::{web, App, Error, HttpResponse, HttpServer, Result};

async fn hello_file() -> Result<HttpResponse, Error> {
    let mut file = File::open("hello.txt")?;
    let mut buffer = "".to_owned();
    let _ = file.read_to_string(&mut buffer);

    Ok(HttpResponse::Ok().body(format!("{}", buffer)))
}
async fn hello() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().body("Hello!!!!"))
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/hello", web::get().to(hello))
            .route("/hello_file", web::get().to(hello_file))
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}
