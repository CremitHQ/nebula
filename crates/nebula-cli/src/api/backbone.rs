use anyhow::Result;
use cached::proc_macro::{cached, io_cached};
use reqwest::IntoUrl;
use serde::{Deserialize, Serialize};
use ulid::Ulid;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SecretResponse {
    pub key: String,
    pub path: String,
    pub cipher: String,
    pub access_condition_ids: Vec<Ulid>,
}

pub async fn get_secrets(backbone_url: impl IntoUrl, path: &str, token: &str) -> Result<Vec<SecretResponse>> {
    let client = reqwest::Client::new();

    let mut url = backbone_url.into_url()?.join("secrets")?;
    url.set_query(Some(&format!("path=/{}", path.trim_matches('/'))));
    let response = client.get(url).bearer_auth(token).send().await?.json::<Vec<SecretResponse>>().await?;

    Ok(response)
}

pub async fn get_secret_with_identifier(
    backbone_url: impl IntoUrl,
    identifier: &str,
    token: &str,
) -> Result<SecretResponse> {
    let client = reqwest::Client::new();

    let url = backbone_url.into_url()?.join("secrets/")?.join(identifier.trim_matches('/'))?;
    let response = client.get(url).bearer_auth(token).send().await?.json::<SecretResponse>().await?;

    Ok(response)
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PostSecretRequest {
    pub path: String,
    pub key: String,
    pub cipher: String,
    pub access_condition_ids: Vec<Ulid>,
}

pub async fn create_secret(backbone_url: impl IntoUrl, request: PostSecretRequest, token: &str) -> Result<()> {
    let client = reqwest::Client::new();

    let url = backbone_url.into_url()?.join("secrets")?;
    let response = client.post(url).bearer_auth(token).json(&request).send().await?;
    response.error_for_status()?;

    Ok(())
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccessConditionResponse {
    pub id: Ulid,
    pub name: String,
    pub expression: String,
}

pub async fn get_access_conditions(backbone_url: impl IntoUrl, token: &str) -> Result<Vec<AccessConditionResponse>> {
    let client = reqwest::Client::new();

    let url = backbone_url.into_url()?.join("policies")?;
    let response = client.get(url).bearer_auth(token).send().await?.json::<Vec<AccessConditionResponse>>().await?;

    Ok(response)
}

#[cached(result = true, key = "String", convert = r#"{ format!("ac:{}/{}", backbone_url.as_str(), id) }"#)]
pub async fn get_access_condition(
    backbone_url: impl IntoUrl,
    id: &Ulid,
    token: &str,
) -> Result<AccessConditionResponse> {
    let client = reqwest::Client::new();

    let url = backbone_url.into_url()?.join(&format!("policies/{id}"))?;
    let response = client.get(url).bearer_auth(token).send().await?.json::<AccessConditionResponse>().await?;

    Ok(response)
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PostPolicyRequest {
    pub name: String,
    pub expression: String,
}

pub async fn create_access_condition(
    backbone_url: impl IntoUrl,
    request: PostPolicyRequest,
    token: &str,
) -> Result<()> {
    let client = reqwest::Client::new();

    let url = backbone_url.into_url()?.join("policies")?;
    let response = client.post(url).bearer_auth(token).json(&request).send().await?;
    response.error_for_status()?;

    Ok(())
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PathResponse {
    pub path: String,
    pub applied_policies: Vec<AppliedPolicy>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppliedPolicy {
    pub expression: String,
    pub allowed_actions: Vec<AllowedAction>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AllowedAction {
    Create,
    Update,
    Delete,
    Manage,
}

pub async fn get_paths(backbone_url: impl IntoUrl, token: &str) -> Result<Vec<PathResponse>> {
    let client = reqwest::Client::new();

    let url = backbone_url.into_url()?.join("paths")?;
    let response = client.get(url).bearer_auth(token).send().await?.json::<Vec<PathResponse>>().await?;

    Ok(response)
}

pub async fn get_path(backbone_url: impl IntoUrl, path: &str, token: &str) -> Result<PathResponse> {
    let client = reqwest::Client::new();

    let url = backbone_url.into_url()?.join(&format!("paths/{path}"))?;
    let response = client.get(url).bearer_auth(token).send().await?.json::<PathResponse>().await?;

    Ok(response)
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePathRequest {
    pub path: String,
    pub applied_policies: Vec<AppliedPolicy>,
}

pub async fn create_path(backbone_url: impl IntoUrl, request: CreatePathRequest, token: &str) -> Result<()> {
    let client = reqwest::Client::new();

    let url = backbone_url.into_url()?.join("paths")?;
    let response = client.post(url).bearer_auth(token).json(&request).send().await?;
    response.error_for_status()?;

    Ok(())
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ParameterResponse {
    pub version: i32,
    pub parameter: String,
}

#[io_cached(
    map_error = "|e| anyhow::anyhow!(e)",
    disk = true,
    time = 600,
    key = "String",
    convert = r#"{ format!("gp:{}", backbone_url.as_str()) }"#
)]
pub async fn get_global_params(backbone_url: impl IntoUrl, token: &str) -> Result<ParameterResponse> {
    let client = reqwest::Client::new();

    let url = backbone_url.into_url()?.join("parameter")?;
    let response = client.get(url).bearer_auth(token).send().await?.json::<ParameterResponse>().await?;

    Ok(response)
}

#[derive(Deserialize, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthorityResponse {
    pub id: Ulid,
    pub name: String,
    pub host: String,
    pub public_key: Option<String>,
}

pub async fn get_authorities(backbone_url: impl IntoUrl, token: &str) -> Result<Vec<AuthorityResponse>> {
    let client = reqwest::Client::new();

    let url = backbone_url.into_url()?.join("authorities")?;
    let response = client.get(url).bearer_auth(token).send().await?.json::<Vec<AuthorityResponse>>().await?;

    Ok(response)
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PostAuthorityRequest {
    pub name: String,
    pub host: String,
}

pub async fn add_authority(backbone_url: impl IntoUrl, request: PostAuthorityRequest, token: &str) -> Result<()> {
    let client = reqwest::Client::new();

    let url = backbone_url.into_url()?.join("authorities")?;
    let response = client.post(url).bearer_auth(token).json(&request).send().await?;
    response.error_for_status()?;

    Ok(())
}
