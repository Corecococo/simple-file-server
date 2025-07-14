use actix_web::{HttpResponse, post, Responder};
use actix_web::web::{Data, Form, Json};
use crate::app_config::AppSettings;
use crate::domain::User;

#[post("/login")]
async fn login(user:Form<User>,data:Data<AppSettings>) -> impl Responder {
    println!("user: {:?}", user);
    let admin_user = &data.admin_user;
    let upload_user = &data.upload_user;
    if user.is_admin(admin_user){
        return HttpResponse::Ok()
            .cookie(actix_web::cookie::Cookie::new("role", "admin"))
            .cookie(actix_web::cookie::Cookie::new("pwd", &admin_user.pwd))
            .finish()
    }else if user.is_upload(upload_user){
        return HttpResponse::Ok()
            .cookie(actix_web::cookie::Cookie::new("role", "upload"))
            .cookie(actix_web::cookie::Cookie::new("pwd", &upload_user.pwd))
            .finish()
    }
    HttpResponse::Forbidden().body("No auth")
}
