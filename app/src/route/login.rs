use crate::app_config::{AppSettings, UserSetting};
use crate::domain::User;
use actix_web::web::{Data, Json};
use actix_web::{HttpResponse, Responder, post};
use serde::Deserialize;
use tracing::instrument;

#[post("/login")]
#[instrument(
    name="Receiver a login request",
    skip(data)
)]
pub async fn login(user: Json<LoginJsonData>, data: Data<AppSettings>) -> impl Responder {
    let admin_user = &data.admin_user;
    let upload_user = &data.upload_user;
    if user.is_admin(admin_user) {
        return HttpResponse::Ok()
            .cookie(actix_web::cookie::Cookie::new("role", "admin"))
            .cookie(actix_web::cookie::Cookie::new("pwd", &admin_user.pwd))
            .finish();
    } else if user.is_upload(upload_user) {
        return HttpResponse::Ok()
            .cookie(actix_web::cookie::Cookie::new("role", "upload"))
            .cookie(actix_web::cookie::Cookie::new("pwd", &upload_user.pwd))
            .finish();
    }
    HttpResponse::Forbidden().body("No auth")
}

#[derive(Deserialize, Debug)]
struct LoginJsonData {
    role: String,
    pwd: String,
}

impl LoginJsonData {
    pub fn is_admin(&self, user_settings: &UserSetting) -> bool {
        self.role == "admin" && self.pwd == user_settings.pwd
    }

    pub fn is_upload(&self, user_settings: &UserSetting) -> bool {
        self.role == "upload" && self.pwd == user_settings.pwd
    }
}
