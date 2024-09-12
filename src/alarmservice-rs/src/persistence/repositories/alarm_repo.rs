use sea_orm::{
    sea_query::Expr, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, FromQueryResult,
    QueryFilter, QuerySelect, SelectModel, Selector,
};

use crate::persistence::entities::alarm::{self};

pub async fn count_active_alarms(
    conn: &DatabaseConnection,
    room_id: i64,
) -> Result<Option<AlarmCount>, DbErr> {
    prepare_count_active_alarms(room_id).one(conn).await
}

fn prepare_count_active_alarms(room_id: i64) -> Selector<SelectModel<AlarmCount>> {
    alarm::Entity::find()
        .select_only()
        .column_as(Expr::col(alarm::Column::Id).count(), "active_alarm_count")
        .filter(alarm::Column::RoomId.eq(room_id))
        .filter(alarm::Column::Acknowledged.eq(false))
        .into_model::<AlarmCount>()
}

#[derive(Clone, Debug, FromQueryResult)]
pub struct AlarmCount {
    pub active_alarm_count: i32,
}
