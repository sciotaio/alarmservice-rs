use axum::{extract::State, http::StatusCode, Json};
use models::models::AlarmDto;

use crate::api::{errors::CustomError, router::AppState};

pub async fn get_alarms_handler(
    _state: State<AppState>,
) -> Result<(StatusCode, Json<Vec<AlarmDto>>), CustomError> {
    let res = vec![AlarmDto {
        reason: Some("Dummy AlarmDto Dto".to_string()),
        ..Default::default()
    }];
    Ok((StatusCode::OK, Json(res)))
}
