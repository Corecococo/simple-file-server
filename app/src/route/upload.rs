use crate::app_config::AppSettings;
use crate::msg;
use actix_multipart::form::MultipartForm;
use actix_multipart::form::tempfile::TempFile;
use actix_web::web::{Data, Json};
use actix_web::{HttpResponse, Responder, post};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::env::current_dir;
use std::fs::{File, metadata};
use std::io::{BufReader, BufWriter, Read, Write};
use std::time::{Duration, SystemTime};
use actix_web::error::HttpError;
use tracing::{error, info, instrument};
use utoipa::{IntoParams, ToSchema};

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
) -> impl Responder {
    let metadata = serde_json::from_str::<Metadata>(form.metadata.as_str());
    if metadata.is_err() {
        return HttpResponse::BadRequest()
            .body(format!("{}", msg!(400, "the metadata is invalid")));
    }

    let metadata = metadata.unwrap();
    let save_file_name = format!(
        "{}_{}_{}",
        metadata.who,
        metadata.purpose,
        form.file.file_name.unwrap()
    );
    let current_path = current_dir().unwrap();
    let create_file_res = File::create(format!("{}", save_file_name));
    if create_file_res.is_err() {
        error!("{:?}", create_file_res);
        return HttpResponse::InternalServerError()
            .body(format!("{}", msg!(500, "create file failed")));
    }

    let save_file = create_file_res.unwrap();
    let mut buf_reader = BufReader::with_capacity(2048, form.file.file);
    let mut buf_writer = BufWriter::with_capacity(2048, save_file);
    let mut buf = [0u8; 2048];
    info!("Saving upload file");
    let start_time_span = SystemTime::now();
    loop {
        let read_count = buf_reader.read(&mut buf).unwrap();
        if read_count == 0 {
            break;
        }
        let _ = buf_writer.write(&buf[..read_count]).unwrap();
    }
    let elapsed = start_time_span
        .elapsed()
        .unwrap_or(Duration::from_secs(0))
        .as_secs();
    info!("Save upload file success, time elapsed {} sec", elapsed);
    HttpResponse::Ok().body(format!("{}", msg!(200, "upload success")))

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
    file: Vec<u8>,
    #[schema(format = "json", value_type = String, example = json!({"who":"coco","purpose":"test"}) )]
    metadata: String,
}
