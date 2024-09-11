use axum::{
    extract::{Query, State},
    http::StatusCode,
};
use serde::Deserialize;

use crate::api::{errors::CustomError, router::AppState};

#[derive(Deserialize)]
pub struct QueryParams {
    name: Option<String>,
}

pub async fn hello_handler(
    Query(q): Query<QueryParams>,
    _state: State<AppState>,
) -> Result<(StatusCode, String), CustomError> {
    Ok((
        StatusCode::OK,
        format!("Hello '{}'", q.name.unwrap_or_else(|| "World".to_string())),
    ))
}
