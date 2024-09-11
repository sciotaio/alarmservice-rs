use sea_orm_migration::prelude::*;

use crate::persistence::entities::{alarm::Alarm, room::Room, schedule::Schedule};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // create 'room' table
        manager
            .create_table(
                Table::create()
                    .if_not_exists()
                    .table(Room::Table)
                    .col(
                        ColumnDef::new(Room::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Room::Name).string().not_null())
                    .to_owned(),
            )
            .await?;

        // create 'schedule' table
        manager
            .create_table(
                Table::create()
                    .if_not_exists()
                    .table(Schedule::Table)
                    .col(
                        ColumnDef::new(Schedule::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Schedule::Begin).integer().not_null())
                    .col(ColumnDef::new(Schedule::End).integer().not_null())
                    .col(ColumnDef::new(Schedule::DaysOfWeekMask).integer().not_null())
                    .col(ColumnDef::new(Schedule::RoomId).big_integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-room-id")
                            .from(Schedule::Table, Schedule::RoomId)
                            .to(Room::Table, Room::Id)
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // create 'alarm' table
        manager
            .create_table(
                Table::create()
                    .if_not_exists()
                    .table(Alarm::Table)
                    .col(
                        ColumnDef::new(Alarm::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Alarm::Reason).string().not_null())
                    .col(ColumnDef::new(Alarm::Acknowledged).boolean().not_null())
                    .col(
                        ColumnDef::new(Alarm::Timestamp)
                            .default(Expr::current_timestamp())
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Alarm::RoomId).big_integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-room-id")
                            .from(Schedule::Table, Schedule::RoomId)
                            .to(Room::Table, Room::Id)
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Room::Table).to_owned())
            .await
    }
}
