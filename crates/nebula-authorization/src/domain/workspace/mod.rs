use std::sync::Arc;

use chrono::Utc;
use nebula_domain::workspace::validate_workspace_name;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DatabaseBackend, DatabaseConnection, DatabaseTransaction, DbErr,
    EntityTrait, PaginatorTrait, QueryFilter, RuntimeErr, SqlxError, Statement,
};
use tracing::info;
use ulid::Ulid;

use crate::database::{migrate_workspace, AuthMethod};

impl WorkspaceService {
    pub(crate) async fn create(&self, transaction: &DatabaseTransaction, name: &str) -> Result<()> {
        use crate::database::workspace::ActiveModel;
        use sea_orm::ActiveValue;
        if !validate_workspace_name(name) {
            return Err(Error::InvalidWorkspaceName);
        }

        self.connection
            .execute(Statement::from_string(
                DatabaseBackend::Postgres,
                format!("CREATE SCHEMA IF NOT EXISTS \"{name}\";"),
            ))
            .await?;

        migrate_workspace(name, &self.database_host, self.database_port, &self.database_name, &self.database_auth)
            .await?;

        if self.exists_by_name(transaction, name).await? {
            return Err(Error::WorkspaceNameConflicted);
        }

        let now = Utc::now();

        ActiveModel {
            id: ActiveValue::Set(Ulid::new().into()),
            name: ActiveValue::Set(name.to_owned()),
            created_at: ActiveValue::Set(now),
            updated_at: ActiveValue::Set(now),
        }
        .insert(transaction)
        .await?;

        info!("workspace(name: {name}) created.");

        Ok(())
    }
}

impl From<DbErr> for Error {
    fn from(value: DbErr) -> Self {
        if let DbErr::Query(RuntimeErr::SqlxError(SqlxError::Database(e))) = value {
            if e.code().as_deref() == Some("23505") {
                Self::WorkspaceNameConflicted
            } else {
                Self::Anyhow(e.into())
            }
        } else {
            Self::Anyhow(value.into())
        }
    }
}
