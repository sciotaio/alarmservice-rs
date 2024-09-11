use sea_orm::entity::prelude::*;
use sea_orm_migration::prelude::*;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "schedule")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i64,
    pub begin: i32,
    pub end: i32,
    #[sea_orm(column_name = "days_of_week_mask")]
    pub days_of_week_mask: i32,
    pub room_id: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::room::Entity",
        from = "Column::RoomId",
        to = "super::room::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Room,
}

impl Related<super::room::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Room.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(DeriveIden)]
pub enum Schedule {
    Table,
    Id,
    Begin,
    End,
    #[sea_orm(iden = "days_of_week_mask")]
    DaysOfWeekMask,
    RoomId,
}
