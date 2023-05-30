use sea_orm_migration::prelude::*;

use super::m20230525_000002_create_projects::Project;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Scope::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Scope::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Scope::Title).string().not_null())
                    .col(ColumnDef::new(Scope::Description).string().not_null())
                    .col(ColumnDef::new(Scope::ProjectId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_project_scope_id")
                            .from(Scope::Table, Scope::ProjectId)
                            .to(Project::Table, Project::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Scope::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
pub enum Scope {
    Table,
    Id,
    Title,
    Description,
    ProjectId,
}
impl Iden for Scope {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        write!(
            s,
            "{}",
            match self {
                Self::Table => "scopes",
                Self::Id => "id",
                Self::Title => "title",
                Self::Description => "description",
                Self::ProjectId => "project_id",
            }
        )
        .unwrap();
    }
}
