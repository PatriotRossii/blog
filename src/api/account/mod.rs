use actix_web::{get, web, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::{services::user_management::LoginService, AppData};

#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct LoginResponse {
    access_token: String,
}

#[get("/login")]
async fn login(data: web::Data<AppData>, info: web::Json<LoginRequest>) -> HttpResponse {
    let access_token = LoginService::try_login(&data.pool, &info.username, &info.password).await;

    if let Ok(access_token) = access_token {
        HttpResponse::Ok().json(LoginResponse { access_token })
    } else {
        HttpResponse::Forbidden().finish()
    }
}
