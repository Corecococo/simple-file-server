use std::net::TcpListener;
use app::startup::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {

    // 读取配置文件
    let app_settings = app::app_config::load_app_config().expect("load app config failed");
    let addr = format!("127.0.0.1:{}", app_settings.port);
    let listener = TcpListener::bind(addr).expect("The port is already used");
    let _ = run(listener)?.await;
    Ok(())
}
