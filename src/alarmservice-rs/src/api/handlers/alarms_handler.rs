use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use models::models::AlarmDto;
use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::{
    api::{errors::CustomError, router::AppState},
    persistence::entities::alarm,
};

#[derive(Deserialize)]
pub struct RoomQueryParams {
    #[serde(rename = "roomId")]
    room_id: i64,
    page: Option<u64>,
    #[serde(rename = "pageSize")]
    page_size: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlarmPage {
    page: u64,
    page_size: u64,
    data: Vec<AlarmDto>,
}

pub async fn get_alarms_for_room_handler(
    Query(q): Query<RoomQueryParams>,
    state: State<AppState>,
) -> Result<(StatusCode, Json<AlarmPage>), CustomError> {
    let page_size = q.page_size.unwrap_or(10);
    let page = q.page.unwrap_or(0);
    let res = alarm::Entity::find()
        .filter(alarm::Column::RoomId.eq(q.room_id))
        .into_model::<alarm::Model>()
        .paginate(&state.conn, page_size)
        .fetch_page(page)
        .await?
        .iter()
        .map(AlarmDto::from)
        .collect();
    Ok((
        StatusCode::OK,
        Json(AlarmPage {
            page,
            page_size,
            data: res,
        }),
    ))
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
