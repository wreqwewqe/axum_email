use axum::response::IntoResponse;
use axum::Json;
use serde_json::json;
pub struct AppError {
    code: u64,
    msg: String,
}

impl AppError {
    pub fn new(code: u64, msg: String) -> Self {
        AppError { code, msg }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        Json(json!({
            "code":self.code,
            "msg":self.msg
        }))
        .into_response()
    }
}
