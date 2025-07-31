use crate::app_config::AppSettings;
use crate::openapi::api_doc::ApiDoc;
use crate::route::health_check::health_check;
use crate::route::login::login;
use crate::route::upload::upload;
use actix_multipart::MultipartError::Payload;
use actix_multipart::form::MultipartFormConfig;
use actix_web::dev::{HttpServiceFactory, Server};
use actix_web::web::{Data, PayloadConfig, ServiceConfig, resource, route};
use actix_web::{App, HttpServer, Route, web};
use std::net::TcpListener;
use std::time::Duration;
use tracing_actix_web::TracingLogger;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

// 启动服务
pub fn run(listener: TcpListener, app_settings: AppSettings) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(move || {
        App::new()
            // .wrap(actix_web::middleware::Logger::default())
            // using tracing logger middleware replace default logger middleware
            // because default logger is used log crate
            //.wrap(crate::middleware::identity::Identity)
            .wrap(TracingLogger::default())
            .app_data(MultipartFormConfig::default().total_limit(1024 * 1024 * 1024 * 10).memory_limit(4 * 1024 * 1024))
            .app_data(Data::new(app_settings.clone()))
            .configure(register_service)
    })
    .workers(2)
    .listen(listener)?
    .run();
    Ok(server)
}

// 注册服务
fn register_service(config: &mut ServiceConfig) {
    config
        .app_data(PayloadConfig::default().limit(10000 * 1000 * 1000 * 10))
        .service(login)
        .service(health_check)
        .service(upload)
        .service(
            SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", ApiDoc::openapi()),
        );
}
