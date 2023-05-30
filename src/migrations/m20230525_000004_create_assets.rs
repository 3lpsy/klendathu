use sea_orm_migration::prelude::*;

use super::m20230525_000003_create_scopes::Scope;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Asset::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Asset::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Asset::Title).string().not_null())
                    .col(ColumnDef::new(Asset::Description).string().not_null())
                    .col(ColumnDef::new(Asset::ScopeId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_scope_scope_id")
                            .from(Asset::Table, Asset::ScopeId)
                            .to(Scope::Table, Scope::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Asset::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
pub enum Asset {
    Table,
    Id,
    Title,
    Description,
    ScopeId,
}
impl Iden for Asset {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        write!(
            s,
            "{}",
            match self {
                Self::Table => "assets",
                Self::Id => "id",
                Self::Title => "title",
                Self::Description => "description",
                Self::ScopeId => "scope_id",
            }
        )
        .unwrap();
    }
}
