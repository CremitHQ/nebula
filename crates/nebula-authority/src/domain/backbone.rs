use anyhow::Result;
use async_trait::async_trait;
use base64::{engine::general_purpose::STANDARD, Engine as _};
use cached::proc_macro::cached;
use nebula_abe::{curves::bn462::Bn462Curve, schemes::isabella24::GlobalParams};
use reqwest::IntoUrl;
use serde::Deserialize;
use url::Url;

#[async_trait]
pub trait BackboneService {
    async fn global_params(&self) -> Result<GlobalParams<Bn462Curve>>;
}

pub struct BackboneClient {
    client: reqwest::Client,
    host: Url,
}

impl BackboneClient {
    pub fn new(host: Url) -> Self {
        Self { host, client: reqwest::Client::new() }
    }

    pub fn client(mut self, client: reqwest::Client) -> Self {
        self.client = client;
        self
    }

    pub fn host(mut self, host: impl IntoUrl) -> anyhow::Result<Self> {
        self.host = host.into_url()?;
        Ok(self)
    }
}

#[derive(Deserialize)]
struct ParameterResponse {
    version: i32,
    parameter: String,
}

impl BackboneClient {
    async fn get_global_params(&self) -> Result<GlobalParams<Bn462Curve>> {
        let url = self.host.join("parameter")?;
        let response = self.client.get(url).send().await?;
        let parameter: ParameterResponse = response.json().await?;
        let parameter = STANDARD.decode(parameter.parameter)?;
        let parameter = rmp_serde::from_slice(&parameter)?;

        Ok(parameter)
    }
}

pub struct BackboneServiceImpl {
    backbone_host: Url,
}

impl BackboneServiceImpl {
    pub fn new(backbone_host: Url) -> Self {
        Self { backbone_host }
    }
}

#[async_trait]
impl BackboneService for BackboneServiceImpl {
    async fn global_params(&self) -> Result<GlobalParams<Bn462Curve>> {
        get_global_params(&self.backbone_host).await
    }
}

#[cached(size = 32, result = true, key = "String", convert = r#"{ format!("{}", host.as_str()) }"#)]
async fn get_global_params(host: &Url) -> Result<GlobalParams<Bn462Curve>> {
    let client = BackboneClient::new(host.clone());
    client.get_global_params().await
}
