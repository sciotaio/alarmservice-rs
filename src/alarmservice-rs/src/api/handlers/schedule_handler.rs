use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use models::models::ScheduleDto;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter};
use serde::Deserialize;

use crate::{
    api::{errors::CustomError, router::AppState},
    persistence::entities::{room, schedule},
};

#[derive(Deserialize)]
pub struct QueryParams {
    #[serde(rename = "roomId")]
    room_id: Option<i64>,
}

pub async fn get_schedules_handler(
    Query(q): Query<QueryParams>,
    state: State<AppState>,
) -> Result<(StatusCode, Json<Vec<ScheduleDto>>), CustomError> {
    let room_id = q
        .room_id
        .ok_or_else(|| CustomError::BadRequest("missing 'roomId'".to_string()))?;

    let room = room::Entity::find_by_id(room_id).one(&state.conn).await;

    let scheds = match room {
        Ok(Some(_existing_room)) => {
            let res = schedule::Entity::find()
                .filter(schedule::Column::RoomId.eq(_existing_room.id))
                .all(&state.conn)
                .await?;
            Ok(res)
        }
        Ok(None) => Err(CustomError::InternalServerError(format!(
            "Room with id '{room_id}' not found"
        ))),
        Err(e) => Err(CustomError::InternalServerError(format!(
            "Could not persist schedule: {}",
            e
        ))),
    }?;

    let mapped = scheds.iter().map(ScheduleDto::from).collect();

    Ok((StatusCode::OK, Json(mapped)))
}

pub async fn post_schedule_handler(
    state: State<AppState>,
    Json(schedule_dto): Json<ScheduleDto>,
) -> Result<StatusCode, CustomError> {
    let mut am = schedule::ActiveModel::try_from(schedule_dto)
        .map_err(|e| CustomError::BadRequest(format!("Invalid schedule: {}", e)))?;

    // check if room with given id exists in db.
    let room_id = am.room_id.take().expect("'room_id' must exist here");
    let room = room::Entity::find_by_id(room_id).one(&state.conn).await;

    match room {
        Ok(Some(_existing_room)) => am.save(&state.conn).await.map_err(|e| {
            CustomError::InternalServerError(format!("Could not persist schedule: {}", e))
        }),
        Ok(None) => Err(CustomError::InternalServerError(format!(
            "Room with id '{room_id}' not found"
        ))),
        Err(e) => Err(CustomError::InternalServerError(format!(
            "Could not persist schedule: {}",
            e
        ))),
    }?;

    Ok(StatusCode::CREATED)
}
