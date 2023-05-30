use sea_orm_migration::prelude::*;

use super::m20230525_000001_create_clients::Client;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Project::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Project::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Project::Title).string().not_null())
                    .col(ColumnDef::new(Project::Description).string().not_null())
                    .col(ColumnDef::new(Project::ClientId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_client_project_id")
                            .from(Project::Table, Project::ClientId)
                            .to(Client::Table, Client::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Project::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
pub enum Project {
    Table,
    Id,
    Title,
    Description,
    ClientId,
}
impl Iden for Project {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        write!(
            s,
            "{}",
            match self {
                Self::Table => "projects",
                Self::Id => "id",
                Self::Title => "title",
                Self::Description => "description",
                Self::ClientId => "client_id",
            }
        )
        .unwrap();
    }
}
