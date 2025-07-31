use std::io::sink;
use std::net::TcpListener;
use once_cell::sync::Lazy;
use app::app_config::load_app_config;
use app::startup::run;
use app::telemetry::{build_tracing_subscriber, init_logger};

pub async fn spawn_app() -> TestApp {
    // let tracing_subscriber = build_tracing_subscriber("test".into(), "info".into());
    // this function only call once , here in multiple threads will be panic
    // init_logger(tracing_subscriber);

    //read config
    let app_settings = load_app_config().expect("Failed to load app config");
    Lazy::force(&TRACING); // force init tracing, it can lazy initial global data safely
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = run(listener, app_settings).expect("Failed to start up server");
    let _ = tokio::spawn(server);
    TestApp(port)
}

pub struct TestApp(pub u16);

static TRACING:Lazy<()> = Lazy::new(||{
    let subscriber = build_tracing_subscriber("test".into(), "debug".into(),sink);
    init_logger(subscriber);
});