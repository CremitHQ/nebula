use std::sync::Arc;

use axum::{
    debug_handler,
    extract::{Path, State},
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::get,
    Extension, Json, Router,
};
use nebula_token::claim::NebulaClaim;

use crate::{
    application::{self, path::PathUseCase, Application},
    server::{check_member_role, router::path::request::PatchPathRequest},
};

use self::request::PostPathRequest;

mod model;
mod request;
mod response;

pub(crate) fn router(application: Arc<Application>) -> axum::Router {
    Router::new()
        .route("/paths", get(handle_get_paths).post(handle_post_path))
        .route("/paths/*path", get(handle_get_path).delete(handle_delete_path).patch(handle_patch_path))
        .route_layer(middleware::from_fn(check_member_role))
        .with_state(application)
}

#[debug_handler]
async fn handle_post_path(
    State(application): State<Arc<Application>>,
    Extension(claim): Extension<NebulaClaim>,
    Json(payload): Json<PostPathRequest>,
) -> Result<impl IntoResponse, application::path::Error> {
    let policies: Vec<_> =
        payload.applied_policies.into_iter().map(nebula_domain::secret::AppliedPolicy::from).collect();
    application.with().path().register(&payload.path, &policies, &claim).await?;

    Ok(StatusCode::NO_CONTENT)
}

#[debug_handler]
async fn handle_get_paths(
    State(application): State<Arc<Application>>,
) -> Result<impl IntoResponse, application::path::Error> {
    let paths = application.with().path().get_all().await?;

    Ok(Json(paths.into_iter().map(response::PathResponse::from).collect::<Vec<_>>()))
}

#[debug_handler]
async fn handle_delete_path(
    Path(path): Path<String>,
    State(application): State<Arc<Application>>,
    Extension(claim): Extension<NebulaClaim>,
) -> Result<impl IntoResponse, application::path::Error> {
    application.with().path().delete(&normalize_path(path), &claim).await?;

    Ok(StatusCode::NO_CONTENT)
}

#[debug_handler]
async fn handle_patch_path(
    Path(path): Path<String>,
    State(application): State<Arc<Application>>,
    Extension(claim): Extension<NebulaClaim>,
    Json(payload): Json<PatchPathRequest>,
) -> Result<impl IntoResponse, application::path::Error> {
    let new_policies: Option<Vec<_>> =
        payload.applied_policies.map(|aps| aps.into_iter().map(nebula_domain::secret::AppliedPolicy::from).collect());
    application
        .with()
        .path()
        .update(&normalize_path(path), payload.path.as_deref(), new_policies.as_deref(), &claim)
        .await?;

    Ok(StatusCode::NO_CONTENT)
}
#[debug_handler]
async fn handle_get_path(
    Path(path): Path<String>,
    State(application): State<Arc<Application>>,
) -> Result<impl IntoResponse, application::path::Error> {
    let path = application.with().path().get(&normalize_path(path)).await?;

    Ok(Json(response::PathResponse::from(path)))
}

fn normalize_path(path: String) -> String {
    if path.starts_with("/") {
        path
    } else {
        format!("/{path}")
    }
}

impl From<application::path::PathData> for response::PathResponse {
    fn from(value: application::path::PathData) -> Self {
        Self {
            path: value.path,
            applied_policies: value.applied_policies.into_iter().map(model::AppliedPolicy::from).collect(),
        }
    }
}

impl From<nebula_domain::secret::AppliedPolicy> for model::AppliedPolicy {
    fn from(value: nebula_domain::secret::AppliedPolicy) -> Self {
        Self {
            expression: value.expression,
            allowed_actions: value.allowed_actions.into_iter().map(model::AllowedAction::from).collect(),
        }
    }
}

impl From<nebula_domain::secret::AllowedAction> for model::AllowedAction {
    fn from(value: nebula_domain::secret::AllowedAction) -> Self {
        match value {
            nebula_domain::secret::AllowedAction::Create => model::AllowedAction::Create,
            nebula_domain::secret::AllowedAction::Update => model::AllowedAction::Update,
            nebula_domain::secret::AllowedAction::Delete => model::AllowedAction::Delete,
            nebula_domain::secret::AllowedAction::Manage => model::AllowedAction::Manage,
        }
    }
}

impl From<model::AppliedPolicy> for nebula_domain::secret::AppliedPolicy {
    fn from(value: model::AppliedPolicy) -> Self {
        Self {
            expression: value.expression,
            allowed_actions: value
                .allowed_actions
                .into_iter()
                .map(nebula_domain::secret::AllowedAction::from)
                .collect(),
        }
    }
}

impl From<model::AllowedAction> for nebula_domain::secret::AllowedAction {
    fn from(value: model::AllowedAction) -> Self {
        match value {
            model::AllowedAction::Create => Self::Create,
            model::AllowedAction::Update => Self::Update,
            model::AllowedAction::Delete => Self::Delete,
            model::AllowedAction::Manage => Self::Manage,
        }
    }
}
