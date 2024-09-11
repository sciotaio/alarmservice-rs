use sea_orm::entity::prelude::*;
use sea_orm_migration::prelude::*;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "room")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i64,
    pub name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::alarm::Entity")]
    Alarm,
    #[sea_orm(has_many = "super::schedule::Entity")]
    Schedule,
}

impl Related<super::alarm::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Alarm.def()
    }
}

impl Related<super::schedule::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Schedule.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(DeriveIden)]
pub enum Room {
    Table,
    Id,
    Name,
}
