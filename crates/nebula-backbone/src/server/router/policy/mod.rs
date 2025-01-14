use std::sync::Arc;

use axum::{
    debug_handler,
    extract::{Path, State},
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::{get, patch, post},
    Json, Router,
};
use ulid::Ulid;

use crate::{
    application::{self, policy::PolicyUseCase, Application},
    server::{check_admin_role, check_member_role},
};

use self::response::PolicyResponse;

mod request;
mod response;

pub(crate) fn router(application: Arc<Application>) -> axum::Router {
    let member_router = Router::new()
        .route("/policies", get(handle_get_policies))
        .route("/policies/:policy_id", get(handle_get_policy))
        .route_layer(middleware::from_fn(check_member_role));
    let admin_router = Router::new()
        .route("/policies", post(handle_post_policy))
        .route("/policies/:policy_id", patch(handle_patch_policy).delete(handle_delete_policy))
        .route_layer(middleware::from_fn(check_admin_role));

    Router::new().merge(member_router).merge(admin_router).with_state(application)
}

#[debug_handler]
async fn handle_get_policies(
    State(application): State<Arc<Application>>,
) -> Result<impl IntoResponse, application::policy::Error> {
    let policies = application.policy().get_all().await?;

    Ok(Json(policies.into_iter().map(response::PolicyResponse::from).collect::<Vec<_>>()))
}
#[debug_handler]

async fn handle_post_policy(
    State(application): State<Arc<Application>>,
    Json(payload): Json<request::PostPolicyRequest>,
) -> Result<impl IntoResponse, application::policy::Error> {
    application.policy().register(&payload.name, &payload.expression).await?;

    Ok(StatusCode::CREATED)
}

#[debug_handler]
async fn handle_get_policy(
    Path(policy_id): Path<Ulid>,
    State(application): State<Arc<Application>>,
) -> Result<impl IntoResponse, application::policy::Error> {
    let policy = application.policy().get_policy(policy_id).await?;

    Ok(Json(PolicyResponse::from(policy)))
}

#[debug_handler]
async fn handle_patch_policy(
    Path(policy_id): Path<Ulid>,
    State(application): State<Arc<Application>>,
    Json(payload): Json<request::PatchPolicyRequest>,
) -> Result<impl IntoResponse, application::policy::Error> {
    application.policy().update(&policy_id, payload.name.as_deref(), payload.expression.as_deref()).await?;

    Ok(StatusCode::NO_CONTENT)
}

#[debug_handler]
async fn handle_delete_policy(
    Path(policy_id): Path<Ulid>,
    State(application): State<Arc<Application>>,
) -> Result<impl IntoResponse, application::policy::Error> {
    application.policy().delete(&policy_id).await?;

    Ok(StatusCode::NO_CONTENT)
}

impl From<application::policy::PolicyData> for response::PolicyResponse {
    fn from(value: application::policy::PolicyData) -> Self {
        Self { id: value.id, name: value.name, expression: value.expression }
    }
}
