use sea_orm::{EnumIter, Iterable};
use sea_orm_migration::{prelude::*, schema::*};

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
                            .integer()
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
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(OpenOptions::SerialPortId)
                            .integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(OpenOptions::Tag).char_len(32).not_null())
                    .col(enumeration(
                        OpenOptions::InitReadState,
                        Alias::new("init_read_state"),
                        InitReadState::iter(),
                    ))
                    .col(ColumnDef::new(OpenOptions::BaudRate).integer().not_null())
                    .col(enumeration(
                        OpenOptions::DataBits,
                        Alias::new("data_bits"),
                        DataBits::iter(),
                    ))
                    .col(enumeration(
                        OpenOptions::FlowControl,
                        Alias::new("flow_control"),
                        FlowControl::iter(),
                    ))
                    .col(enumeration(
                        OpenOptions::Parity,
                        Alias::new("parity"),
                        Parity::iter(),
                    ))
                    .col(enumeration(
                        OpenOptions::StopBits,
                        Alias::new("stop_bits"),
                        StopBits::iter(),
                    ))
                    .col(
                        ColumnDef::new(OpenOptions::TimeoutMilliSecs)
                            .integer()
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

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(OpenOptions::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(SerialPort::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(Iden, EnumIter)]
enum InitReadState {
    Read,
    Stop,
}

#[derive(Iden, EnumIter)]
pub enum DataBits {
    Five,
    Six,
    Seven,
    Eight,
}

#[derive(Iden, EnumIter)]
pub enum FlowControl {
    None,
    Software,
    Hardware,
}

#[derive(Iden, EnumIter)]
pub enum Parity {
    None,
    Odd,
    Even,
}

#[derive(Iden, EnumIter)]
pub enum StopBits {
    One,
    Two,
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
