use models::models::ScheduleDto;
use sea_orm::Set;

use crate::persistence::entities::schedule;

impl TryFrom<ScheduleDto> for schedule::ActiveModel {
    type Error = String;

    fn try_from(value: ScheduleDto) -> Result<Self, Self::Error> {
        let begin = value.begin.map_or_else(
            || Err("missing 'begin' field"),
            |val| match (val.cmp(&0), val.cmp(&1439)) {
                (std::cmp::Ordering::Less, _) => {
                    Err("field 'begin' out of range (min: 0, max: 1439)")
                }
                (_, std::cmp::Ordering::Greater) => {
                    Err("field 'begin' out of range (min: 0, max: 1439)")
                }
                (_, _) => Ok(val),
            },
        )?;
        let end = value.end.map_or_else(
            || Err("missing 'end' field"),
            |val| match (val.cmp(&0), val.cmp(&1439)) {
                (std::cmp::Ordering::Less, _) => {
                    Err("field 'end' out of range (min: 0, max: 1439)")
                }
                (_, std::cmp::Ordering::Greater) => {
                    Err("field 'end' out of range (min: 0, max: 1439)")
                }
                (_, _) => Ok(val),
            },
        )?;
        let days_of_week_mask = value.days_of_week_mask.map_or_else(
            || Err("missing 'days_of_week_mask' field"),
            |val| match (val.cmp(&0), val.cmp(&127)) {
                (std::cmp::Ordering::Less, _) => {
                    Err("field 'days_of_week_mask' out of range (min: 0, max: 127)")
                }
                (_, std::cmp::Ordering::Greater) => {
                    Err("field 'days_of_week_mask' out of range (min: 0, max: 127)")
                }
                (_, _) => Ok(val),
            },
        )?;

        let room_id = value
            .room_id
            .map_or_else(|| Err("missing 'room_id' field"), |val| Ok(val))?;

        Ok(schedule::ActiveModel {
            begin: Set(begin),
            end: Set(end),
            days_of_week_mask: Set(days_of_week_mask),
            room_id: Set(room_id),
            ..Default::default()
        })
    }
}

impl From<&schedule::Model> for ScheduleDto {
    fn from(value: &schedule::Model) -> Self {
        ScheduleDto {
            begin: Some(value.begin),
            end: Some(value.end),
            days_of_week_mask: Some(value.days_of_week_mask),
            room_id: Some(value.room_id),
        }
    }
}