use anyhow::Result;
use reqwest::IntoUrl;
use serde::Deserialize;

const TOKEN_HEADER: &str = "Token";

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct MachineIdentityTokenResponse {
    pub access_token: String,
}

pub async fn get_token_from_machine_identity_token(authz_url: impl IntoUrl, token: &str) -> Result<String> {
    let client = reqwest::Client::new();

    let url = authz_url.into_url()?.join("machine-identities/login")?;
    let response =
        client.get(url).header(TOKEN_HEADER, token).send().await?.json::<MachineIdentityTokenResponse>().await?;

    Ok(response.access_token)
}
