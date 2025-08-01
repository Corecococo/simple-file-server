
pub mod download;
pub mod health_check;
pub mod login;
pub mod upload;

#[macro_export]
macro_rules! msg {
    ($code:expr,$msg:expr,$data:tt) => {
        format!("{}",json!({
            "code":$code,
            "msg":$msg,
            "data":$data
        }))

    };
    ($code:expr,$msg:expr)=>{
        format!("{}",json!({
            "code":$code,
            "msg":$msg,
            "data":null
        }))

    }
}
