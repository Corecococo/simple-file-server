use crate::app_config::AppSettings;
use crate::route::health_check::health_check;
use crate::route::login::login;
use actix_web::dev::Server;
use actix_web::web::{Data, ServiceConfig};
use actix_web::{App, HttpServer};
use std::net::TcpListener;
use actix_web::middleware::DefaultHeaders;
use tracing_actix_web::TracingLogger;

// 启动服务
pub fn run(listener: TcpListener, app_settings: AppSettings) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(move || {
        App::new()
            // .wrap(actix_web::middleware::Logger::default())
            // using tracing logger middleware replace default logger middleware
            // because default logger is used log crate
            .wrap(TracingLogger::default())
            .app_data(Data::new(app_settings.clone()))
            .configure(register_service)
    })
    .listen(listener)?
    .run();
    Ok(server)
}

// 注册服务
fn register_service(config: &mut ServiceConfig) {
    config.service(login).service(health_check);
}
