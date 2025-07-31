use serde_json::json;

pub mod download;
pub mod health_check;
pub mod login;
pub mod upload;

#[macro_export]
macro_rules! msg {
    ($code:expr,$msg:expr,$data:tt) => {
        json!({
            "code":$code,
            "msg":$msg,
            "data":$data
        })
    };
    ($code:expr,$msg:expr)=>{
        json!({
            "code":$code,
            "msg":$msg,
            "data":null
        })
    }
}
