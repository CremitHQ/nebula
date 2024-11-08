use std::path::PathBuf;

use config::{Config, File, FileFormat};
use directories::BaseDirs;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub(crate) struct ApplicationConfig {
    pub port: u16,
}

pub(super) fn load_config(
    path_override: Option<PathBuf>,
    port_override: Option<u16>,
) -> anyhow::Result<ApplicationConfig> {
    let config_file_path = if let Some(path_override) = path_override {
        path_override
    } else {
        let base_dirs = BaseDirs::new().expect("Failed to get base directories");
        let user_config_dir = base_dirs.config_dir();
        let tessera_config_dir = user_config_dir.join("tessera");
        if !tessera_config_dir.exists() {
            std::fs::create_dir_all(&tessera_config_dir)?;
        }
        tessera_config_dir.join("idp_config.toml")
    };

    let config: ApplicationConfig = Config::builder()
        .add_source(File::from(config_file_path).format(FileFormat::Toml))
        .set_default("port", 9000)?
        .set_override_option("port", port_override)?
        .build()?
        .try_deserialize()?;

    Ok(config)
}
