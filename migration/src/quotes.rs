use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Quote::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Quote::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Quote::CreatedAt)
                            .date()
                            .extra("DEFAULT (datetime('now', 'localtime'))".to_owned()),
                    )
                    .col(ColumnDef::new(Quote::GuildId).string().not_null())
                    .col(ColumnDef::new(Quote::QuotedBy).string().not_null())
                    .col(ColumnDef::new(Quote::QuoteString).string().not_null())
                    .col(ColumnDef::new(Quote::Author).string().not_null())
                    .col(ColumnDef::new(Quote::AuthorAvatarUrl).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Quote::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden

enum Quote {
    Table,
    Id,
    CreatedAt,
    GuildId,
    QuotedBy,
    QuoteString,
    Author,
    AuthorAvatarUrl,
}

impl Iden for Quote {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        write!(
            s,
            "{}",
            match self {
                Self::Table => "quotes",
                Self::Id => "id",
                Self::CreatedAt => "created_at",
                Self::GuildId => "guild_id",
                Self::QuotedBy => "quoted_by",
                Self::QuoteString => "quote_string",
                Self::Author => "author",
                Self::AuthorAvatarUrl => "author_avatar_url",
            }
        )
        .unwrap();
    }
}
