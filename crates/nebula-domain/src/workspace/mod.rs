use async_trait::async_trait;

mod workspace_service;

use sea_orm::{DatabaseTransaction, EntityTrait};
use tracing::info;
use ulid::Ulid;
#[cfg(any(test, feature = "testing"))]
pub use workspace_service::MockWorkspaceService;
pub use workspace_service::{WorkspaceService, WorkspaceServiceImpl};

use crate::database::{Persistable, UlidId};

#[derive(Debug, PartialEq)]
pub struct Workspace {
    id: Ulid,
    pub name: String,
    deleted: bool,
}

impl Workspace {
    pub fn new(id: Ulid, name: String) -> Self {
        Self { id, name, deleted: false }
    }

    pub fn delete(&mut self) {
        self.deleted = true
    }
}

pub fn validate_workspace_name(name: &str) -> bool {
    if name.is_empty() || name.len() > 255 || !name.chars().all(|c| c.is_ascii_alphanumeric()) {
        return false;
    }

    true
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("workspace name already exists")]
    WorkspaceNameConflicted,

    #[error("invalid workspace name")]
    InvalidWorkspaceName,

    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

#[async_trait]
impl Persistable for Workspace {
    type Error = crate::workspace::Error;

    async fn persist(self, transaction: &DatabaseTransaction) -> crate::workspace::Result<()> {
        if self.deleted {
            use crate::database::workspace::Entity;

            Entity::delete_by_id(UlidId::from(self.id)).exec(transaction).await?;

            let workspace_name = self.name;
            info!("workspace(name: {workspace_name}) is deleted.");
            return Ok(());
        };

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::sync::Arc;

    use chrono::Utc;
    use sea_orm::{DatabaseBackend, DbErr, MockDatabase, MockExecResult, TransactionTrait};
    use ulid::Ulid;

    use super::{Error, WorkspaceService, WorkspaceServiceImpl};

    #[tokio::test]
    async fn when_insert_is_successful_then_workspace_service_returns_ok() {
        use crate::database::workspace::Model;

        const WORKSPACE_NAME: &str = "testworkspace";
        let now = Utc::now();
        let mock_database = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results([[maplit::btreemap! {
                "num_items" => sea_orm::Value::BigInt(Some(0))
            }]])
            .append_query_results([vec![Model {
                id: Ulid::new().into(),
                name: WORKSPACE_NAME.to_owned(),
                created_at: now,
                updated_at: now,
            }]])
            .append_exec_results([MockExecResult { last_insert_id: 0, rows_affected: 0 }]);
        let mock_connection = Arc::new(mock_database.into_connection());

        let workspace_service = WorkspaceServiceImpl::default();

        let transaction = mock_connection.begin().await.expect("begining transaction should be successful");

        let result = workspace_service.create(&transaction, WORKSPACE_NAME).await;

        transaction.commit().await.expect("commiting transaction should be successful");

        result.expect("creating workspace should be successful");
    }

    #[tokio::test]
    async fn when_workspace_already_exists_then_workspace_service_returns_workspace_name_conflicted_error() {
        const WORKSPACE_NAME: &str = "testworkspace";
        let mock_database = MockDatabase::new(DatabaseBackend::Postgres)
            .append_exec_results([
                MockExecResult { last_insert_id: 0, rows_affected: 1 },
                MockExecResult { last_insert_id: 0, rows_affected: 1 },
            ])
            .append_query_results([[maplit::btreemap! {
                "num_items" => sea_orm::Value::BigInt(Some(1))
            }]]);
        let mock_connection = Arc::new(mock_database.into_connection());

        let workspace_service = WorkspaceServiceImpl::default();

        let transaction = mock_connection.begin().await.expect("begining transaction should be successful");

        let result = workspace_service.create(&transaction, WORKSPACE_NAME).await;

        transaction.commit().await.expect("commiting transaction should be successful");

        assert!(matches!(result, Err(Error::WorkspaceNameConflicted)));
    }

    #[tokio::test]
    async fn when_insert_is_failed_then_workspace_service_returns_anyhow_err() {
        const WORKSPACE_NAME: &str = "testworkspace";
        let mock_database = MockDatabase::new(DatabaseBackend::Postgres)
            .append_exec_results([
                MockExecResult { last_insert_id: 0, rows_affected: 0 },
                MockExecResult { last_insert_id: 0, rows_affected: 0 },
            ])
            .append_query_errors(vec![DbErr::Custom("some error".to_owned())]);
        let mock_connection = Arc::new(mock_database.into_connection());

        let workspace_service = WorkspaceServiceImpl::default();

        let transaction = mock_connection.begin().await.expect("begining transaction should be successful");

        let result = workspace_service.create(&transaction, WORKSPACE_NAME).await;

        transaction.commit().await.expect("commiting transaction should be successful");

        assert!(matches!(result, Err(Error::Anyhow(_))));
        assert_eq!(result.err().unwrap().to_string(), "Custom Error: some error");
    }

    #[tokio::test]
    async fn when_getting_workspaces_is_successful_then_workspace_service_returns_workspaces_ok() {
        use crate::database::workspace::Model;

        const WORKSPACE_NAME: &str = "testworkspace";
        let now = Utc::now();
        let mock_database = MockDatabase::new(DatabaseBackend::Postgres).append_query_results([vec![Model {
            id: Ulid::new().into(),
            name: WORKSPACE_NAME.to_owned(),
            created_at: now,
            updated_at: now,
        }]]);
        let mock_connection = Arc::new(mock_database.into_connection());

        let workspace_service = WorkspaceServiceImpl::default();

        let transaction = mock_connection.begin().await.expect("begining transaction should be successful");

        let result = workspace_service.get_all(&transaction).await;
        transaction.commit().await.expect("commiting transaction should be successful");

        assert_eq!(result.expect("creating workspace should be successful")[0].name, WORKSPACE_NAME);
    }

    #[tokio::test]
    async fn when_getting_workspaces_is_failed_then_workspace_service_returns_anyhow_err() {
        let mock_database = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_errors(vec![DbErr::Custom("some error".to_owned())]);
        let mock_connection = Arc::new(mock_database.into_connection());

        let workspace_service = WorkspaceServiceImpl::default();

        let transaction = mock_connection.begin().await.expect("begining transaction should be successful");

        let result = workspace_service.get_all(&transaction).await;
        transaction.commit().await.expect("commiting transaction should be successful");

        assert!(matches!(result, Err(Error::Anyhow(_))));
        assert_eq!(result.err().unwrap().to_string(), "Custom Error: some error");
    }

    #[tokio::test]
    async fn when_getting_not_existing_workspace_then_workspace_service_returns_ok_of_none() {
        use crate::database::workspace::Model;
        const WORKSPACE_NAME: &str = "testworkspace";
        let mock_database = MockDatabase::new(DatabaseBackend::Postgres).append_query_results([Vec::<Model>::new()]);
        let mock_connection = Arc::new(mock_database.into_connection());

        let workspace_service = WorkspaceServiceImpl::default();

        let transaction = mock_connection.begin().await.expect("begining transaction should be successful");

        let result = workspace_service.get_by_name(&transaction, WORKSPACE_NAME).await;
        transaction.commit().await.expect("commiting transaction should be successful");

        assert_eq!(result.expect("creating workspace should be successful"), None);
    }

    #[tokio::test]
    async fn when_getting_workspace_is_failed_then_workspace_service_returns_anyhow_err() {
        const WORKSPACE_NAME: &str = "testworkspace";
        let mock_database = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_errors(vec![DbErr::Custom("some error".to_owned())]);
        let mock_connection = Arc::new(mock_database.into_connection());

        let workspace_service = WorkspaceServiceImpl::default();

        let transaction = mock_connection.begin().await.expect("begining transaction should be successful");

        let result = workspace_service.get_by_name(&transaction, WORKSPACE_NAME).await;
        transaction.commit().await.expect("commiting transaction should be successful");

        assert!(matches!(result, Err(Error::Anyhow(_))));
        assert_eq!(result.err().unwrap().to_string(), "Custom Error: some error");
    }

    #[tokio::test]
    async fn when_getting_workspace_is_succeed_then_workspace_service_returns_ok_of_workspace() {
        use crate::database::workspace::Model;
        const WORKSPACE_NAME: &str = "testworkspace";

        let now = Utc::now();
        let mock_database = MockDatabase::new(DatabaseBackend::Postgres).append_query_results([vec![Model {
            id: Ulid::new().into(),
            name: WORKSPACE_NAME.to_owned(),
            created_at: now,
            updated_at: now,
        }]]);
        let mock_connection = Arc::new(mock_database.into_connection());

        let workspace_service = WorkspaceServiceImpl::default();

        let transaction = mock_connection.begin().await.expect("begining transaction should be successful");

        let result = workspace_service.get_by_name(&transaction, WORKSPACE_NAME).await;
        transaction.commit().await.expect("commiting transaction should be successful");

        assert_eq!(
            result.expect("getting workspace should be successful").expect("workspace should be exists").name,
            WORKSPACE_NAME
        )
    }
}
