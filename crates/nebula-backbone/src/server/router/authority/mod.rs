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
    application::{
        self,
        authority::{AuthorityData, AuthorityUseCase},
        Application,
    },
    server::{check_admin_role, check_member_role, response::handle_internal_server_error},
};

use self::{
    request::{PatchAuthorityRequest, PostAuthorityRequest},
    response::AuthorityResponse,
};

mod request;
mod response;

pub(crate) fn router(application: Arc<Application>) -> axum::Router {
    let member_router = Router::new()
        .route("/authorities", get(handle_get_authorities))
        .route("/authorities/:authority_id", get(handle_get_authority))
        .route_layer(middleware::from_fn(check_member_role));
    let admin_router = Router::new()
        .route("/authorities", post(handle_post_authority))
        .route("/authorities/:authority_id", patch(handle_patch_authority).delete(handle_delete_authority))
        .route_layer(middleware::from_fn(check_admin_role));

    Router::new().merge(member_router).merge(admin_router).with_state(application)
}

impl IntoResponse for application::authority::Error {
    fn into_response(self) -> axum::response::Response {
        match self {
            application::authority::Error::Anyhow(e) => handle_internal_server_error(&*e).into_response(),
            application::authority::Error::NameAlreadyInUse { entered_authority_name } => {
                response::AuthorityNameAlreadyInUseErrorResponse { entered_authority_name }.into_response()
            }
            application::authority::Error::AuthorityNotExists { entered_authority_id } => {
                response::AuthorityNotFoundResponse { entered_authority_id }.into_response()
            }
        }
    }
}

impl From<AuthorityData> for response::AuthorityResponse {
    fn from(value: AuthorityData) -> Self {
        Self { id: value.id, name: value.name, host: value.host, public_key: value.public_key }
    }
}

#[debug_handler]
async fn handle_post_authority(
    State(application): State<Arc<Application>>,
    Json(payload): Json<PostAuthorityRequest>,
) -> application::authority::Result<impl IntoResponse> {
    application.authority().register_authority(&payload.name, &payload.host).await?;
    Ok(StatusCode::CREATED)
}

#[debug_handler]
async fn handle_get_authorities(
    State(application): State<Arc<Application>>,
) -> application::authority::Result<impl IntoResponse> {
    let authorities = application.authority().get_authorities().await?;
    let payload: Vec<_> = authorities.into_iter().map(response::AuthorityResponse::from).collect();

    Ok(Json(payload))
}

#[debug_handler]
async fn handle_get_authority(
    Path(authority_id): Path<Ulid>,
    State(application): State<Arc<Application>>,
) -> application::authority::Result<impl IntoResponse> {
    let authority = application.authority().get_authority(&authority_id).await?;

    let payload = AuthorityResponse::from(authority);

    Ok(Json(payload))
}

#[debug_handler]
async fn handle_patch_authority(
    Path(authority_id): Path<Ulid>,
    State(application): State<Arc<Application>>,
    Json(payload): Json<PatchAuthorityRequest>,
) -> application::authority::Result<impl IntoResponse> {
    application
        .authority()
        .update_authority(&authority_id, payload.name.as_deref(), payload.public_key.as_deref())
        .await?;

    Ok(StatusCode::NO_CONTENT)
}

#[debug_handler]
async fn handle_delete_authority(
    Path(authority_id): Path<Ulid>,
    State(application): State<Arc<Application>>,
) -> application::authority::Result<impl IntoResponse> {
    application.authority().delete_authority(&authority_id).await?;

    Ok(StatusCode::NO_CONTENT)
}
