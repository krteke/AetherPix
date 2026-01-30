use axum::{http::StatusCode, response::IntoResponse, Json};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("邮箱‘{0}’已被注册")]
    EmailAlreadyExist(String),
    #[error("用户名不存在")]
    UsernameNotExist,
    #[error("用户名或密码错误")]
    WrongCredentials,
    #[error("此用户邮箱没有通过验证，请检查收件箱并点击链接激活账号")]
    EmailNotVerified,
    #[error(transparent)]
    LocoError(#[from] loco_rs::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> loco_rs::prelude::Response {
        let (status, message) = match &self {
            AppError::EmailAlreadyExist(e) => {
                (StatusCode::BAD_REQUEST, format!("邮箱{}已被占用", e))
            }
            AppError::UsernameNotExist => (StatusCode::UNAUTHORIZED, "用户名不存在".to_string()),
            AppError::WrongCredentials => (StatusCode::BAD_REQUEST, "用户名或密码错误".to_string()),
            AppError::EmailNotVerified => (
                StatusCode::UNAUTHORIZED,
                "此用户邮箱没有通过验证，请检查收件箱并点击链接激活账号".to_string(),
            ),
            AppError::LocoError(e) => {
                tracing::error!("{}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "服务器内部错误".to_string(),
                )
            }
            _ => {
                todo!()
            }
        };

        let body = Json(serde_json::json!({
            "error": status.to_string(),
            "description": message
        }));

        (status, body).into_response()
    }
}

pub type AppResult<T> = Result<T, AppError>;
