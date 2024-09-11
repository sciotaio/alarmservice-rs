use sea_orm::{ActiveModelTrait, EntityTrait, Set};
use sea_orm_migration::prelude::*;

use crate::persistence::entities::{room, schedule};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // ################
        // ### Rooms ###
        // ################
        // create a list of room models
        let models: Vec<room::ActiveModel> = (0..100)
            .map(|i| room::ActiveModel {
                name: Set(format!("room_{i:04}").to_string()),
                ..Default::default()
            })
            .collect();
        // insert into db
        room::Entity::insert_many(models)
            .exec(manager.get_connection())
            .await
            .map(|_insres| ())?;

        // ################
        // ### Schedule ###
        // ################
        // get room from DB
        let room = room::Entity::find_by_id(1)
            .one(manager.get_connection())
            .await?
            .expect("Room with ID 1 not in DB!");
        // create schedule for room
        schedule::ActiveModel {
            begin: Set(0),                        // 00:00
            end: Set(1439),                       // 23:59
            days_of_week_mask: Set(0b01111111i32), // all days of the week
            room_id: Set(room.id),
            ..Default::default()
        }
        .save(manager.get_connection())
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // clean all inserted rooms
        room::Entity::delete_many()
            .exec(manager.get_connection())
            .await
            .map(|_delres| ())
    }
}
