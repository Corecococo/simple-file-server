use actix_web::web::Data;
use actix_web::{App, HttpServer};
use app::route::login;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // 读取配置文件
    let app_settings = app::app_config::load_app_config().expect("load app config failed");
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(app_settings.clone()))
            .service(login)
    })
    .bind("127.0.0.1:8889")?
    .run()
    .await
}
