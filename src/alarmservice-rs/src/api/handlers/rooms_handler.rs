use axum::{extract::State, http::StatusCode, Json};
use models::models::RoomDto;
use sea_orm::EntityTrait;

use crate::{api::{errors::CustomError, router::AppState}, persistence::entities::room};

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
