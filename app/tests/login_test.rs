use crate::common::spawn_app::spawn_app;
use quickcheck::Arbitrary;
use quickcheck_macros::quickcheck;
use serde::Serialize;

mod common;
#[tokio::test]
async fn admin_user_login_success() {
    let app = spawn_app().await;
    let admin_json = r#"
    {
        "role": "admin",
        "pwd": "admin"
    }
    "#;
    let test_url = format!("http://127.0.0.1:{}/login", app.0);
    let client = reqwest::Client::new();
    let res = client
        .post(test_url)
        .header("Content-Type", "application/json")
        .body(admin_json)
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
}

#[tokio::test]
async fn upload_user_login_success() {
    let app = spawn_app().await;
    let upload_json = r#"
    {
        "role": "upload",
        "pwd": "upload"
    }
    "#;
    let test_url = format!("http://127.0.0.1:{}/login", app.0);
    let client = reqwest::Client::new();
    let res = client
        .post(test_url)
        .header("Content-Type", "application/json")
        .body(upload_json)
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
}

#[tokio::test]
async fn no_auth_login_fail() {
    let app = spawn_app().await;
    let mut g = quickcheck::Gen::new(10);
    let test_url = format!("http://127.0.0.1:{}/login", app.0);
    let client = reqwest::Client::new();
    // 测试100次数非法登录
    for _ in 0..100 {
        let user = TestUser::arbitrary(&mut g);
        let json_data = serde_json::to_string(&user).unwrap();
        let res = client
            .post(&test_url)
            .header("Content-Type", "application/json")
            .body(json_data)
            .send()
            .await
            .unwrap();
        assert_eq!(res.status(), 403)
    }
}

#[derive(Clone, Serialize)]
struct TestUser {
    role: String,
    pwd: String,
}

impl Arbitrary for TestUser {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        let role = String::arbitrary(g);
        let pwd = String::arbitrary(g);
        TestUser { role, pwd }
    }
}
