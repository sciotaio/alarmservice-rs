use axum::{extract::State, http::StatusCode, Json};
use models::models::RoomDto;

use crate::api::{errors::CustomError, router::AppState};

pub async fn get_rooms_handler(
    _state: State<AppState>,
) -> Result<(StatusCode, Json<Vec<RoomDto>>), CustomError> {
    let res = vec![RoomDto {
        name: Some("Dummy Room Dto".to_string()),
        ..Default::default()
    }];
    Ok((StatusCode::OK, Json(res)))
}
