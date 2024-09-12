use axum::{extract::{Path, Query, State}, http::StatusCode, Json};
use models::models::AlarmDto;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, Set};
use serde::Deserialize;
use tracing::info;

use crate::{api::{errors::CustomError, router::AppState}, persistence::entities::alarm};

#[derive(Deserialize)]
pub struct QueryParams {
    #[serde(rename = "roomId")]
    room_id: i64,
}

pub async fn get_alarms_handler(
    Query(q): Query<QueryParams>,
    state: State<AppState>,
) -> Result<(StatusCode, Json<Vec<AlarmDto>>), CustomError> {
    let res = alarm::Entity::find()
        .filter(alarm::Column::RoomId.eq(q.room_id))
        .all(&state.conn)
        .await?
        .iter()
        .map(AlarmDto::from)
        .collect();
    Ok((StatusCode::OK, Json(res)))
}

pub async fn acknowledge_alarm_handler(
    Path(alarm_id): Path<i64>,
    state: State<AppState>,
) -> Result<StatusCode, CustomError> {
    info!("[alarms_handler] acknowledge alarm with id '{alarm_id}'");

    alarm::Entity::find_by_id(alarm_id)
        .one(&state.conn)
        .await
        .map_err(|e| {
            CustomError::InternalServerError(format!("Could not query alarm from DB: {e}"))
        })?
        .ok_or_else(|| CustomError::NotFound)
        .map(|model| {
            let active_model: alarm::ActiveModel = model.into();
            active_model
        })
        .and_then(|mut db_alarm| {
            db_alarm.acknowledged = Set(true);
            Ok(db_alarm)
        })
        .map(|acknowledged_alarm| async {
            alarm::Entity::update(acknowledged_alarm)
                .exec(&state.conn)
                .await
        })?
        .await
        .map_err(|e| {
            CustomError::InternalServerError(format!("Could not acknowledge alarm in DB: {e}"))
        })?;

    Ok(StatusCode::OK)
}
