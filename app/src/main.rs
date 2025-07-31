use app::startup::run;
use app::telemetry::init_logger;
use std::net::TcpListener;
use tracing_appender::non_blocking;
use tracing_appender::rolling::daily;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // read config
    let app_settings = app::app_config::load_app_config().expect("load app config failed");

    // create non-blocking writer
    let file_appender = daily("./logs/", "daily.log");
    // be careful: _guard must have enough lifetime, because when it is dropped, logs will not be written to file
    // so we have to use non_blocking function in the main function
    let (non_blocking, _guard) = non_blocking(file_appender);

    // init logger
    let tracing_subscriber = app::telemetry::build_tracing_subscriber(
        "simple-file-server".into(),
        "info".into(),
        non_blocking,
    );
    init_logger(tracing_subscriber);
    // init listener
    let addr = format!("127.0.0.1:{}", app_settings.port);
    let listener = TcpListener::bind(addr).expect("The port is already used");
    let _ = run(listener, app_settings)?.await;
    Ok(())
}
