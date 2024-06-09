use actix_web::{
    error::ErrorInternalServerError,
    web::{self, Data},
    App, HttpResponse, HttpServer,
};
use awc::Client;
use serde::{Deserialize, Serialize};
use tera::Tera;

#[derive(Serialize, Deserialize, Debug)]
pub struct Tutor {
    pub tutor_id: i32,
    pub tutor_name: String,
    pub tutor_pic_url: String,
    pub tutor_profile: String,
}

async fn handle_get_tutors(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, actix_web::Error> {
    let client = Client::default();
    let response = client
        .get("http://localhost:3000/tutors/")
        .send()
        .await
        .unwrap()
        .body()
        .await
        .unwrap();

    let str_list = std::str::from_utf8(&response.as_ref()).unwrap();
    let tutors: Vec<Tutor> = serde_json::from_str(str_list).unwrap();
    let mut ctx = tera::Context::new();
    ctx.insert("tutors", &tutors);
    let rendered_html = tmpl
        .render("list.html", &ctx)
        .map_err(|_| ErrorInternalServerError("Template error"))?;

    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(rendered_html))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Listening on: 127.0.0.1:8080");
    HttpServer::new(|| {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/static/iter4/**/*")).unwrap();

        App::new()
            .app_data(Data::new(tera))
            .service(actix_files::Files::new("/static", "./static").show_files_listing())
            .service(web::resource("/tutor").route(web::get().to(handle_get_tutors)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
