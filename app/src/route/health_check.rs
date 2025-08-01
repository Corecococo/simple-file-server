use crate::msg;
use actix_web::{Error, HttpResponse, get};
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
    Ok(HttpResponse::Ok().body(msg!(200, "server is running")))
}
