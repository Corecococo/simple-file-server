use crate::route::health_check::health_check;
use crate::route::login::login;
use actix_web::dev::Server;
use actix_web::web::ServiceConfig;
use actix_web::{App, HttpServer};
use std::net::TcpListener;

// 启动服务
pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| App::new()
        .wrap(actix_web::middleware::Logger::default())
        .configure(register_service))
        .listen(listener)?
        .run();
    Ok(server)
}

// 注册服务
fn register_service(config: &mut ServiceConfig) {
    config.service(login).service(health_check);
}
