use actix_web::{Error, HttpResponse, Responder, get};
use serde_json::json;

#[utoipa::path(
    get,
    path = "/health",
    tag = "Health Check",
    responses(
        (status = 200, description = "Health check success")
    )
)]
#[get("/health")]
pub async fn health_check() -> Result<HttpResponse, Error> {
    let json_msg = json!({
        "code": 200,
        "msg": "Server is running"
    });
    Ok(HttpResponse::Ok().body(format!("{}", json_msg)))
}
