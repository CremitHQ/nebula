use std::sync::Arc;

use axum::{
    debug_handler,
    extract::{Path, Query, State},
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::get,
    Extension, Json, Router,
};
use base64::{prelude::BASE64_STANDARD, Engine};
use nebula_token::claim::NebulaClaim;
use serde::Deserialize;

use crate::{
    application::{
        self,
        secret::{SecretData, SecretRegisterCommand, SecretUpdate, SecretUseCase},
        Application,
    },
    server::check_member_role,
};

use self::{
    request::{PatchSecretRequest, PostSecretRequest},
    response::{InvalidSecretCipherResponse, SecretResponse},
};

mod request;
mod response;

pub(crate) fn router(application: Arc<Application>) -> axum::Router {
    Router::new()
        .route("/secrets", get(handle_get_secrets).post(handle_post_secret))
        .route(
            "/secrets/*secret_identifier",
            get(handle_get_secret).delete(handle_delete_secret).patch(handle_patch_secret),
        )
        .route_layer(middleware::from_fn(check_member_role))
        .with_state(application)
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
struct GetSecretsApiQueryParam {
    path: Option<String>,
}

#[debug_handler]
async fn handle_get_secrets(
    Query(query_params): Query<GetSecretsApiQueryParam>,
    State(application): State<Arc<Application>>,
    Extension(claim): Extension<NebulaClaim>,
) -> Result<impl IntoResponse, application::secret::Error> {
    let secrets = application.with().secret().list(query_params.path.as_deref().unwrap_or("/"), &claim).await?;
    let response: Vec<SecretResponse> = secrets.into_iter().map(SecretResponse::from).collect();

    Ok(Json(response))
}

#[debug_handler]
async fn handle_post_secret(
    State(application): State<Arc<Application>>,
    Extension(claim): Extension<NebulaClaim>,
    Json(payload): Json<PostSecretRequest>,
) -> Result<impl IntoResponse, application::secret::Error> {
    let cipher = if let Ok(cipher) = BASE64_STANDARD.decode(payload.cipher) {
        cipher
    } else {
        return Ok(InvalidSecretCipherResponse {}.into_response());
    };

    application
        .with()
        .secret()
        .register(
            SecretRegisterCommand {
                path: payload.path,
                key: payload.key,
                cipher,
                access_condition_ids: payload.access_condition_ids,
            },
            &claim,
        )
        .await?;

    Ok(StatusCode::CREATED.into_response())
}

#[debug_handler]
async fn handle_get_secret(
    Path(secret_identifier): Path<String>,
    State(application): State<Arc<Application>>,
    Extension(claim): Extension<NebulaClaim>,
) -> Result<impl IntoResponse, application::secret::Error> {
    let secret = application.with().secret().get(&format!("/{secret_identifier}"), &claim).await?;

    Ok(Json(SecretResponse::from(secret)))
}

#[debug_handler]
async fn handle_delete_secret(
    Path(secret_identifier): Path<String>,
    State(application): State<Arc<Application>>,
    Extension(claim): Extension<NebulaClaim>,
) -> Result<impl IntoResponse, application::secret::Error> {
    application.with().secret().delete(&format!("/{secret_identifier}"), &claim).await?;

    Ok(StatusCode::NO_CONTENT)
}

#[debug_handler]
async fn handle_patch_secret(
    Path(secret_identifier): Path<String>,
    State(application): State<Arc<Application>>,
    Extension(claim): Extension<NebulaClaim>,
    Json(payload): Json<PatchSecretRequest>,
) -> Result<impl IntoResponse, application::secret::Error> {
    let cipher = match payload.cipher.map(|cipher| BASE64_STANDARD.decode(cipher)) {
        Some(Ok(cipher)) => Some(cipher),
        Some(Err(_)) => return Ok(InvalidSecretCipherResponse {}.into_response()),
        None => None,
    };

    application
        .with()
        .secret()
        .update(
            &format!("/{secret_identifier}"),
            SecretUpdate { path: payload.path, cipher, access_condition_ids: payload.access_condition_ids },
            &claim,
        )
        .await?;

    Ok(StatusCode::NO_CONTENT.into_response())
}

impl From<SecretData> for SecretResponse {
    fn from(value: SecretData) -> Self {
        Self {
            key: value.key,
            path: value.path,
            cipher: BASE64_STANDARD.encode(value.cipher),
            access_condition_ids: value.access_condition_ids,
        }
    }
}
