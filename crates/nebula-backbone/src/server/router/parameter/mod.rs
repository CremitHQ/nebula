use std::sync::Arc;

use axum::{
    extract::State,
    middleware,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use base64::{engine::general_purpose::STANDARD, Engine as _};

use crate::{
    application::{
        self,
        parameter::{ParameterData, ParameterUseCase},
        Application,
    },
    server::check_admin_role,
};

use self::response::ParameterResponse;

mod response;

pub(crate) fn public_router(application: Arc<Application>) -> axum::Router {
    Router::new().route("/parameter", get(handle_get_parameter)).with_state(application)
}

pub(crate) fn router(application: Arc<Application>) -> axum::Router {
    let admin_router = Router::new()
        .route("/parameter", post(handle_post_parameter))
        .route_layer(middleware::from_fn(check_admin_role));
    Router::new().merge(admin_router).with_state(application)
}

async fn handle_post_parameter(
    State(application): State<Arc<Application>>,
) -> Result<impl IntoResponse, application::parameter::Error> {
    let parameter = application.with().parameter().create().await?;
    let response: ParameterResponse = parameter.try_into()?;

    Ok(Json(response))
}

async fn handle_get_parameter(
    State(application): State<Arc<Application>>,
) -> Result<impl IntoResponse, application::parameter::Error> {
    let parameter = application.with().parameter().get().await?;
    let response: ParameterResponse = parameter.try_into()?;

    Ok(Json(response))
}

impl TryFrom<ParameterData> for ParameterResponse {
    type Error = application::parameter::Error;

    fn try_from(value: ParameterData) -> Result<Self, Self::Error> {
        let parameter = rmp_serde::to_vec(&value.value)?;
        let parameter = STANDARD.encode(&parameter);
        Ok(Self { version: value.version, parameter })
    }
}
