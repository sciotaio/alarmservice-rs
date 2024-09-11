use std::collections::HashMap;

use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;
use tracing::warn;

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq)]
pub enum CustomError {
    BadRequest(String),
    Conflict(String),
    ConflictWithBody(String, HashMap<String, String>),
    Unauthorized(String),
    InternalServerError(String),
    NotFound,
}

impl IntoResponse for CustomError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_msg, err_body) = match self {
            Self::BadRequest(msg) => log_and_return(StatusCode::BAD_REQUEST, msg.to_string(), None),
            Self::Conflict(msg) => log_and_return(StatusCode::CONFLICT, msg.to_string(), None),
            Self::ConflictWithBody(msg, body) => {
                log_and_return(StatusCode::CONFLICT, msg.to_string(), Some(body))
            }
            Self::Unauthorized(msg) => {
                log_and_return(StatusCode::UNAUTHORIZED, msg.to_string(), None)
            }
            Self::InternalServerError(msg) => {
                log_and_return(StatusCode::INTERNAL_SERVER_ERROR, msg.to_string(), None)
            }
            Self::NotFound => log_and_return(StatusCode::NOT_FOUND, "Not found".to_string(), None),
        };

        let mut error_msg = HashMap::from([("error".to_string(), error_msg)]);
        if err_body.is_some() {
            for (key, value) in err_body.unwrap().iter() {
                error_msg.insert(key.to_owned(), value.to_owned());
            }
        }

        (status, Json(json!(error_msg))).into_response()
    }
}

fn log_and_return(
    status: StatusCode,
    error_message: String,
    body: Option<HashMap<String, String>>,
) -> (StatusCode, String, Option<HashMap<String, String>>) {
    warn!("{}", error_message);
    (status, error_message, body)
}

impl From<sea_orm::DbErr> for CustomError {
    fn from(error: sea_orm::DbErr) -> CustomError {
        CustomError::InternalServerError(format!("DB Error: {}", error))
    }
}
