use std::path::{Path, PathBuf};

use crate::prelude::*;

const TARGET: &str = "mural_server::config";

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Config {
    #[serde(default = "default_port")]
    pub port: u16,
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_home_path = Self::config_home_path()?;
        let _ = std::fs::create_dir_all(&config_home_path);
        let config_file_path = config_home_path.join("config.toml");

        info!(target: TARGET, "loading configuration from '{}'", config_file_path.display());
        let config_file_content = match std::fs::read_to_string(config_file_path) {
            Ok(config_file_content) => config_file_content,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                info!("config file does not exist; using default configuration");
                "".to_string()
            }
            Err(e) => return Err(Error::ConfigRead(e)),
        };

        Ok(toml::from_str(&config_file_content)?)
    }

    fn config_home_path() -> Result<PathBuf> {
        std::env::var("MURAL_SERVER_CONFIG_HOME")
            .map(|raw_file_path| Path::new(&raw_file_path).to_path_buf())
            .or(xdg::BaseDirectories::with_prefix("mural-server")
                .map(|base_dirs| base_dirs.get_config_home()))
            .map_err(|_| Error::ConfigHome)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self { port: 46666 }
    }
}

fn default_port() -> u16 {
    Config::default().port
}
