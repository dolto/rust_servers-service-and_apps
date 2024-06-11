use actix_web::{web, HttpResponse};
use argon2::Config;
use awc::Client;
use serde_json::json;

use crate::{
    dbaccess::{get_user_record, post_new_user},
    errors::EzyTutorError,
    models::{TutorRegisterForm, TutorResponse, User},
    states::AppState,
};

pub async fn show_register_form(
    tmpl: web::Data<tera::Tera>,
) -> Result<HttpResponse, actix_web::error::Error> {
    let mut ctx = tera::Context::new();
    ctx.insert("error", "");
    ctx.insert("current_username", "");
    ctx.insert("current_password", "");
    ctx.insert("current_confirmation", "");
    ctx.insert("current_imageurl", "");
    ctx.insert("current_profile", "");

    let s = tmpl
        .render("register.html", &ctx)
        .map_err(|_| EzyTutorError::TeraError("Template error".to_string()))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

pub async fn handle_register(
    tmpl: web::Data<tera::Tera>,
    app_state: web::Data<AppState>,
    params: web::Form<TutorRegisterForm>,
) -> Result<HttpResponse, actix_web::error::Error> {
    let mut ctx = tera::Context::new();
    let s;
    let username = params.username.clone();
    let user = get_user_record(&app_state.db, username.clone()).await;
    let user_not_found = user.is_err();
    if user_not_found {
        if params.password != params.confirmation {
            ctx.insert("error", "Passwords do not match");
            ctx.insert("current_username", &username);
            ctx.insert("current_password", "");
            ctx.insert("current_confirmation", "");
            ctx.insert("current_imageurl", &params.imageurl);
            ctx.insert("current_profile", &params.profile);
            s = tmpl
                .render("register.html", &ctx)
                .map_err(|_| EzyTutorError::TeraError("Template error".to_string()))?;
        } else {
            let new_tutor = json!({
                "tutor_name": username.clone(),
                "tutor_pic_url": params.imageurl.clone(),
                "tutor_profile": params.profile.clone()
            });
            let awc_client = Client::default();
            let res = awc_client
                .post("http://localhost:3000/tutors/")
                .send_json(&new_tutor)
                .await
                .unwrap()
                .body()
                .await?;
            let tutor_response: TutorResponse = serde_json::from_str(&std::str::from_utf8(&res)?)?;
            s = format!("Congratulations. your to successful regist");

            // 비밀번호 해싱
            let salt = b"somerandomsalt";
            let config = Config::default();
            let hash =
                argon2::hash_encoded(params.password.clone().as_bytes(), salt, &config).unwrap();

            // 유저 등록
            let user = User {
                username: tutor_response.tutor_name,
                tutor_id: Some(tutor_response.tutor_id),
                user_password: hash,
            };
            let _ = post_new_user(&app_state.db, user).await?;
        }
    } else {
        ctx.insert("error", "User Id already exists");
        ctx.insert("current_username", &username);
        ctx.insert("current_password", "");
        ctx.insert("current_confirmation", "");
        ctx.insert("current_imageurl", &params.imageurl);
        ctx.insert("current_profile", &params.profile);
        s = tmpl
            .render("register.html", &ctx)
            .map_err(|_| EzyTutorError::TeraError("Template error".to_string()))?;
    }
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}
