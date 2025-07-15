use app::spawn_app::spawn_app;

#[tokio::test]
async fn admin_user_login_success() {
    let app = spawn_app().await;
    let admin_json = r#"
    {
        "username": "admin",
        "password": "admin"
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
        "username": "upload",
        "password": "upload"
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
    let user_json = r#"
    {
        "username": "upload",
        "password": "upload1"
    }
    "#;
    let test_url = format!("http://127.0.0.1:{}/login", app.0);
    let client = reqwest::Client::new();
    let res = client
        .post(test_url)
        .header("Content-Type", "application/json")
        .body(user_json)
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 403)
}
