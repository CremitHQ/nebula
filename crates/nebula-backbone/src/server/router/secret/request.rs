use serde::Deserialize;
use ulid::Ulid;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostSecretRequest {
    pub path: String,
    pub key: String,
    pub cipher: String,
    pub access_condition_ids: Vec<Ulid>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PatchSecretRequest {
    pub path: Option<String>,
    pub cipher: Option<String>,
    pub access_condition_ids: Option<Vec<Ulid>>,
}
