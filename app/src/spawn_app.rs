use std::net::TcpListener;
use crate::startup::run;

pub async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = run(listener).expect("Failed to start up server");
    let _ = tokio::spawn(server);
    TestApp(port)
}

pub struct TestApp(pub u16);
