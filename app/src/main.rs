use app::startup::run;
use app::telemetry::init_logger;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // read config
    let app_settings = app::app_config::load_app_config().expect("load app config failed");
    // init logger
    let tracing_subscriber =
        app::telemetry::build_tracing_subscriber("simple-file-server".into(), "info".into());
    init_logger(tracing_subscriber);
    // init listener
    let addr = format!("127.0.0.1:{}", app_settings.port);
    let listener = TcpListener::bind(addr).expect("The port is already used");
    let _ = run(listener, app_settings)?.await;
    Ok(())
}
