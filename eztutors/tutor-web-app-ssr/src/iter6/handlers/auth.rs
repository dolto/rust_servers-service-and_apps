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

pub async fn show_signin_form(
    tmpl: web::Data<tera::Tera>,
) -> Result<HttpResponse, actix_web::error::Error> {
    let mut ctx = tera::Context::new();
    ctx.insert("error", "");
    ctx.insert("current_name", "");
    ctx.insert("current_password", "");

    let s = tmpl
        .render("signin.html", &ctx)
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

pub async fn handle_signin(
    app_state: web::Data<AppState>,
    tmpl: web::Data<tera::Tera>,
    params: web::Form<User>,
) -> Result<HttpResponse, actix_web::error::Error> {
    let s;
    let mut ctx = tera::Context::new();
    let user = get_user_record(&app_state.db, params.username.clone()).await;

    if let Ok(user) = user {
        let password_match =
            argon2::verify_encoded(&user.user_password.trim(), &params.user_password.as_bytes())
                .unwrap();

        if password_match {
            ctx.insert("name", &params.username);
            ctx.insert("title", "Signin confirmation!");
            ctx.insert("message", "You have successfully looged in to EzyTutor!");

            s = tmpl
                .render("user.html", &ctx)
                .map_err(|_| EzyTutorError::TeraError("Template Error".to_string()))?;
        } else {
            ctx.insert("error", "Rejection Login");
            ctx.insert("current_name", &params.username);
            ctx.insert("current_password", &params.user_password);

            s = tmpl
                .render("signin.html", &ctx)
                .map_err(|_| EzyTutorError::TeraError("Template error".to_string()))?;
        }
    } else {
        ctx.insert("error", "Cant Found User!");
        ctx.insert("current_name", &params.username);
        ctx.insert("current_password", &params.user_password);

        s = tmpl
            .render("signin.html", &ctx)
            .map_err(|_| EzyTutorError::TeraError("Template error".to_string()))?;
    }
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}
