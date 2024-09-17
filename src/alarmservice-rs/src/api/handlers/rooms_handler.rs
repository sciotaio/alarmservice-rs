use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use models::models::RoomDto;
use sea_orm::EntityTrait;
use serde::{Deserialize, Serialize};

use crate::{
    api::{errors::CustomError, router::AppState},
    persistence::{entities::room, repositories::alarm_repo},
};

pub async fn get_rooms_handler(
    state: State<AppState>,
) -> Result<(StatusCode, Json<Vec<RoomDto>>), CustomError> {
    let res = room::Entity::find()
        .all(&state.conn)
        .await?
        .iter()
        .map(RoomDto::from)
        .collect();
    Ok((StatusCode::OK, Json(res)))
}

pub async fn active_alarm_count_handler(
    Path(room_id): Path<i64>,
    state: State<AppState>,
) -> Result<(StatusCode, Json<AlarmCountDto>), CustomError> {
    let count = alarm_repo::count_active_alarms(&state.conn, room_id)
        .await
        .map_err(|db_err| CustomError::InternalServerError(format!("Could not query DB: {db_err}")))
        .and_then(|a| a.ok_or_else(|| CustomError::NotFound))
        .map(|a| AlarmCountDto {
            active_alarm_count: a.active_alarm_count,
        })?;

    Ok((StatusCode::OK, Json(count)))
}

#[derive(Serialize, Deserialize)]
pub struct AlarmCountDto {
    pub active_alarm_count: i64,
}
