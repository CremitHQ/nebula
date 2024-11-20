use config::{Config, File, FileFormat};
use directories::BaseDirs;
use serde::Deserialize;

use crate::{domain::token::jwk::jwk_set::JwkSet, Args};

#[derive(Deserialize, Debug)]
pub(crate) struct ApplicationConfig {
    pub port: u16,
    pub base_url: String,
    pub database: DatabaseConfig,
    pub upstream_idp: UpstreamIdpConfig,
    pub token: TokenConfig,
}

#[derive(Deserialize, Debug)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub database_name: String,
    pub auth: DatabaseAuthConfig,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "method", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DatabaseAuthConfig {
    Credential { username: String, password: Option<String> },
    RdsIamAuth { username: String },
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UpstreamIdpConfig {
    Saml(SAMLConfig),
}

#[derive(Deserialize, Debug)]
pub struct SAMLConfig {
    pub entity_id: Option<String>,
    pub sso_url: String,
    pub idp_issuer: String,
    pub ca: String,
    pub claims: ClaimsConfig,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ClaimsConfig {
    All,
    Mapping(Vec<(String, String)>),
}

#[derive(Deserialize, Debug)]
pub struct TokenConfig {
    pub lifetime: u64,
    pub jwks: Option<JwkSet>,
    pub jwk_kid: Option<String>,
}

pub(super) fn load_config(args: Args) -> anyhow::Result<ApplicationConfig> {
    let config_file_path = if let Some(path_override) = args.config {
        path_override
    } else {
        let base_dirs = BaseDirs::new().expect("Failed to get base directories");
        let user_config_dir = base_dirs.config_dir();
        let nebula_config_dir = user_config_dir.join("nebula");
        if !nebula_config_dir.exists() {
            std::fs::create_dir_all(&nebula_config_dir)?;
        }
        nebula_config_dir.join("authorization_config.toml")
    };

    let config: ApplicationConfig = Config::builder()
        .add_source(File::from(config_file_path).format(FileFormat::Toml))
        .set_override_option("port", args.port.map(|port| port.to_string()))?
        .set_override_option("base_url", args.base_url)?
        .set_override_option("database.host", args.database_host)?
        .set_override_option("database.port", args.database_port)?
        .set_override_option("database.database_name", args.database_name)?
        .set_override_option("database.auth.username", args.database_username)?
        .set_override_option("database.auth.password", args.database_password)?
        .set_default("token.lifetime", 6 * 3600)?
        .build()?
        .try_deserialize()?;

    Ok(config)
}