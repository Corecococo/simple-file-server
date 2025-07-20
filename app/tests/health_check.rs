use crate::common::spawn_app::spawn_app;

mod common;

#[tokio::test]
async fn health_check()  {
    // 创建测试服务器
    let app = spawn_app().await;
    let test_url = format!("http://127.0.0.1:{}/health", app.0);
    let client = reqwest::Client::new();
    let res = client.get(test_url).send().await.unwrap();
    assert_eq!(res.status(), 200);
}