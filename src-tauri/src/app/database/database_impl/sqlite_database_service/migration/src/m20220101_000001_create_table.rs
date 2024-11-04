use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(SerialPort::Table)
                    .col(
                        ColumnDef::new(SerialPort::Id)
                            .unsigned()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(SerialPort::Name)
                            .char_len(32)
                            .not_null()
                            .unique_key(),
                    )
                    .to_owned(),
            )
            .await?;

        // Index on Name column
        manager
            .create_index(
                Index::create()
                    .table(SerialPort::Table)
                    .col(SerialPort::Name)
                    .name("idx_serial_port_name")
                    .unique()
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(OpenOptions::Table)
                    .col(
                        ColumnDef::new(OpenOptions::Id)
                            .unsigned()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(OpenOptions::SerialPortId)
                            .unsigned()
                            .not_null(),
                    )
                    .col(ColumnDef::new(OpenOptions::Tag).char_len(32).not_null())
                    .col(
                        ColumnDef::new(OpenOptions::InitReadState)
                            .small_unsigned()
                            .not_null(),
                    )
                    .col(ColumnDef::new(OpenOptions::BaudRate).unsigned().not_null())
                    .col(
                        ColumnDef::new(OpenOptions::DataBits)
                            .small_unsigned()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(OpenOptions::FlowControl)
                            .small_unsigned()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(OpenOptions::Parity)
                            .small_unsigned()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(OpenOptions::StopBits)
                            .small_unsigned()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(OpenOptions::TimeoutMilliSecs)
                            .unsigned()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Cascade)
                            .from(OpenOptions::Table, OpenOptions::SerialPortId)
                            .to(SerialPort::Table, SerialPort::Id),
                    )
                    .to_owned(),
            )
            .await?;

        // Index on Tag column
        manager
            .create_index(
                Index::create()
                    .table(OpenOptions::Table)
                    .col(OpenOptions::Tag)
                    .name("idx_open_options_tag")
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Packet::Table)
                    .col(
                        ColumnDef::new(Packet::Id)
                            .unsigned()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Packet::SerialPortId).unsigned().not_null())
                    .col(ColumnDef::new(Packet::Tag).char_len(32).not_null())
                    .col(
                        ColumnDef::new(Packet::Timestamp)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(ColumnDef::new(Packet::Incoming).boolean().not_null())
                    .col(ColumnDef::new(Packet::Outgioing).boolean().not_null())
                    .col(ColumnDef::new(Packet::OutgoingDirect).boolean())
                    .col(ColumnDef::new(Packet::OutgoingBroadcast).boolean())
                    // represents the tag of the subscription
                    .col(ColumnDef::new(Packet::OutgoingSubscription).char_len(32))
                    .col(ColumnDef::new(Packet::Data).blob().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Cascade)
                            .from(Packet::Table, Packet::SerialPortId)
                            .to(SerialPort::Table, SerialPort::Id),
                    )
                    .to_owned(),
            )
            .await?;

        // Index on Tag column
        manager
            .create_index(
                Index::create()
                    .table(Packet::Table)
                    .col(OpenOptions::Tag)
                    .name("idx_packet_tag")
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(OpenOptions::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(SerialPort::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Packet::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum OpenOptions {
    Table,
    Id,
    SerialPortId,
    Tag,
    InitReadState,
    BaudRate,
    DataBits,
    FlowControl,
    Parity,
    StopBits,
    TimeoutMilliSecs,
}

#[derive(DeriveIden)]
enum SerialPort {
    Table,
    Id,
    Name,
}

#[derive(DeriveIden)]
enum Packet {
    Table,
    Id,
    SerialPortId,
    Tag,
    Timestamp,
    Incoming,
    Outgioing,
    OutgoingDirect,
    OutgoingBroadcast,
    OutgoingSubscription,
    Data,
}
