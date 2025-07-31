use crate::app_config::{AppSettings, UserSetting};
use crate::middleware::identity::Claim;
use crate::msg;
use actix_web::web::{Data, Json};
use actix_web::{HttpResponse, Responder, post};
use serde::Deserialize;
use serde_json::json;
use std::time::Duration;
use tracing::{info, instrument, warn};
use utoipa::ToSchema;

#[utoipa::path(
    post,
    tag = "Login",
    path = "/login",
    responses(
        (status = 200, description = "login success"),
        (status = "5XX" , description = "server error"),
        (status = 401, description = "login fail")
    ),
)]
#[post("/login")]
#[instrument(
    name = "Receiver a login request",
    skip(data),
    fields(
        username = %user.role,
        password = %user.pwd
    )
)]
pub async fn login(user: Json<LoginDao>, data: Data<AppSettings>) -> impl Responder {
    let admin_user = &data.admin_user;
    let upload_user = &data.upload_user;
    let mut claim = Claim::default();
    if user.is_admin(admin_user) {
        info!("Admin login success");
        let jwt = claim.generate_token(Duration::from_secs(1800), Some("admin".into()));
        let json_msg = msg!(200,"login success",{
            "token":jwt,
            "role":"admin"
        });
        return HttpResponse::Ok().body(format!("{}", json_msg));
    } else if user.is_upload(upload_user) {
        info!("Upload login success");
        let jwt = claim.generate_token(Duration::from_secs(1800), Some("upload".into()));
        let json_msg = msg!(200,"login success",{
            "token": jwt,
            "role": "upload"
        });
        return HttpResponse::Ok().body(format!("{}", json_msg));
    }
    warn!("A failed login request");
    HttpResponse::Unauthorized().body(format!("{}", msg!(401, "Your name or password is error")))
}

#[derive(Deserialize, Debug, ToSchema)]
struct LoginDao {
    /// user role
    role: String,
    /// user password
    pwd: String,
}

impl LoginDao {
    pub fn is_admin(&self, user_settings: &UserSetting) -> bool {
        self.role == "admin" && self.pwd == user_settings.pwd
    }

    pub fn is_upload(&self, user_settings: &UserSetting) -> bool {
        self.role == "upload" && self.pwd == user_settings.pwd
    }
}
