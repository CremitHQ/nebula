use async_trait::async_trait;
use sea_orm::DbErr;
use sea_orm_migration::{IntoSchemaManagerConnection, MigrationTrait, MigratorTrait};

use super::AuthMethod;

mod m20241126_001_init_backbone;

pub struct Migrator;

#[async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20241126_001_init_backbone::Migration)]
    }
}

#[cfg(test)]
pub async fn migrate_backbone(_host: &str, _port: u16, _database_name: &str, _auth: &AuthMethod) -> anyhow::Result<()> {
    use tracing::debug;

    debug!("workspace migration not supported in test environment");

    Ok(())
}

#[cfg(not(test))]
pub async fn migrate_backbone(host: &str, port: u16, database_name: &str, auth: &AuthMethod) -> anyhow::Result<()> {
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
