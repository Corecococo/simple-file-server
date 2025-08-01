use crate::app_config::AppSettings;
use crate::msg;
use actix_multipart::form::MultipartForm;
use actix_multipart::form::tempfile::TempFile;
use actix_web::error::{ErrorBadRequest, ErrorInternalServerError};
use actix_web::web::Data;
use actix_web::{Error, HttpResponse, post};
use serde::Deserialize;
use serde_json::json;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader, BufWriter};
use tracing::{error, info, instrument};
use utoipa::ToSchema;

#[utoipa::path(
    tag = "Upload",
    path = "/home/upload",
    request_body(
        content = UploadApiDto,
        content_type = "multipart/form-data",
    ),
    responses(
        (status = 200, description = "upload success"),
        (status = 400, description = "bad request"),
        (status = 500, description = "internal server error")
    )
)]
#[post("/home/upload")]
#[instrument(
    name = "Received a upload request",
    fields(
        metadata = %form.metadata.to_string()
    ),
    skip(data)
)]
pub async fn upload(
    MultipartForm(form): MultipartForm<UploadDto>,
    data: Data<AppSettings>,
) -> Result<HttpResponse, Error> {
    let metadata = serde_json::from_str::<Metadata>(form.metadata.as_str()).map_err(|e| {
        error!("{:?}", e);
        ErrorBadRequest(msg!(400, "the metadata is invalid"))
    })?;

    let save_file_name = format!(
        "{}_{}_{}",
        metadata.who,
        metadata.purpose,
        form.file.file_name.unwrap_or("unknow".to_string())
    );

    let target_dir = data.target_dir.clone();
    let save_file = File::create(format!("{}/{}", target_dir, save_file_name))
        .await
        .map_err(|e| {
            error!("{:?}", e);
            ErrorInternalServerError(msg!(500, "internal server error"))
        })?;

    let mut buf_reader = BufReader::with_capacity(2048, File::from_std(form.file.file.into_file()));
    let mut buf_writer = BufWriter::with_capacity(2048, save_file);
    let mut buf = [0u8; 2048];
    info!("Saving upload file");
    loop {
        let read_count = buf_reader.read(&mut buf).await.map_err(|e| {
            error!("{:?}", e);
            ErrorInternalServerError(msg!(500, "internal server error"))
        })?;
        if read_count == 0 {
            break;
        }
        let _ = buf_writer.write(&buf[..read_count]).await.map_err(|e| {
            error!("{:?}", e);
            ErrorInternalServerError(msg!(500, "internal server error"))
        })?;
    }
    info!("Save upload file success");
    Ok(HttpResponse::Ok().body(msg!(200, "upload success")))
}

#[derive(Debug, MultipartForm)]
struct UploadDto {
    file: TempFile,
    metadata: actix_multipart::form::text::Text<String>,
}

#[derive(Debug, Deserialize)]
struct Metadata {
    /// 谁上传
    who: String,

    /// 用途
    purpose: String,
}

#[derive(Debug, ToSchema, Deserialize)]
struct UploadApiDto {
    #[schema(format = "binary", value_type = String)]
    #[allow(dead_code)]
    file: Vec<u8>,
    #[schema(format = "json", value_type = String, example = json!({"who":"coco","purpose":"test"})
    )]
    #[allow(dead_code)]
    metadata: String,
}
