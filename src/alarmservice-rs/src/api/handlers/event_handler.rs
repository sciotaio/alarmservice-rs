use std::cmp::Ordering;

use ::chrono::Timelike;
use axum::{extract::State, http::StatusCode, Json};
use models::models::EventDto;
use sea_orm::{
    sqlx::types::chrono::{DateTime, Utc},
    ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter,
};
use tracing::{info, warn};

use crate::{
    api::{errors::CustomError, router::AppState},
    persistence::entities::{alarm, room, schedule},
};

pub async fn new_event_handler(
    state: State<AppState>,
    Json(event_dto): Json<EventDto>,
) -> Result<StatusCode, CustomError> {
    // get room
    let room = room::Entity::find_by_id(event_dto.room_id)
        .one(&state.conn)
        .await?
        .ok_or_else(|| CustomError::NotFound)?;

    // parse timestamp (and possibly fail) or use Utc::now()
    let event_ts_actual = event_dto.clone().timestamp.map_or_else(
        || Ok::<DateTime<Utc>, CustomError>(Utc::now()), // sorry, if you didn't provide a timestamp, i'm going to use my own time here!
        |ts| {
            let parsed = DateTime::parse_from_rfc3339(ts.as_str())
                .map_err(|e| {
                    CustomError::InternalServerError(format!(
                        "Could not parse field 'timestamp': {e}"
                    ))
                })?
                .with_timezone(&Utc);
            Ok(parsed)
        },
    )?;
    let mins_of_day = event_ts_actual.hour() * 60 + event_ts_actual.minute();

    // load schedule(s) for our room and check the event against it.
    let scheds = schedule::Entity::find()
        .filter(schedule::Column::RoomId.eq(room.id))
        .all(&state.conn)
        .await?;

    let is_alert = scheds.iter().any(|sched| {
        let begin = sched.begin as u32;
        let end = sched.end as u32;
        let a = begin.cmp(&end);
        let b = begin.cmp(&mins_of_day);
        let c = end.cmp(&mins_of_day);

        let res = match (a, b, c) {
            (Ordering::Less, Ordering::Less | Ordering::Equal, Ordering::Greater) => true,
            (Ordering::Less, _, _) => false,
            (Ordering::Greater | Ordering::Equal, _, _) => {
                warn!("Malformed schedule with id '{}'", sched.id);
                false
            }
        };
        res
    });

    if is_alert {
        warn!(
            "Event '{}' for room '{}' caused an alert! Saving alert to DB ...",
            event_dto.event_type, event_dto.room_id
        );
        // insert alarm in db
        alarm::ActiveModel::from((event_dto.to_owned().into(), event_ts_actual))
            .save(&state.conn)
            .await?;
    } else {
        info!(
            "Event '{}' for room '{}' didn't caused an alert!",
            event_dto.event_type, event_dto.room_id
        );
    }

    Ok(StatusCode::CREATED)
}
