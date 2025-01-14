use async_trait::async_trait;
use sea_orm::{
    sea_query::{Alias, IntoIden},
    DbErr,
};
use sea_orm_migration::{IntoSchemaManagerConnection, MigrationTrait, MigratorTrait};

use super::AuthMethod;

mod m20241128_001_init_authorization;

pub struct Migrator;

#[async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20241128_001_init_authorization::Migration)]
    }

    fn migration_table_name() -> sea_orm::DynIden {
        Alias::new("seaql_migrations_authorization").into_iden()
    }
}

pub async fn migrate_authorization(
    host: &str,
    port: u16,
    database_name: &str,
    auth: &AuthMethod,
) -> anyhow::Result<()> {
    let connection = super::connect_to_database(host, port, database_name, auth).await?;

    migrate(connection.as_ref()).await?;

    Ok(())
}

async fn migrate<'d, D>(db: D) -> Result<(), DbErr>
where
    D: IntoSchemaManagerConnection<'d>,
{
    Migrator::up(db, None).await
}
