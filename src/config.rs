use std::path::{Path, PathBuf};

use crate::prelude::*;

const TARGET: &str = "mural_server::config";

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Config {
    #[serde(default = "default_port")]
    pub port: u16,

    #[serde(skip)]
    pub wallpaper_paths: Vec<PathBuf>,
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

        let mut config: Config = toml::from_str(&config_file_content)?;
        config.wallpaper_paths = Self::wallpaper_paths()?;
        Ok(config)
    }

    fn wallpaper_paths() -> Result<Vec<PathBuf>> {
        let data_home_path = Self::config_home_path()?;
        let wallpapers_path = data_home_path.join("wallpapers");
        let _ = std::fs::create_dir_all(&wallpapers_path);
        info!(target: TARGET, "loading wallpapers from '{}'", wallpapers_path.display());

        let wallpaper_paths = std::fs::read_dir(&wallpapers_path)
            .unwrap()
            .collect::<Result<Vec<std::fs::DirEntry>, _>>()
            .unwrap()
            .iter()
            .map(|dir_entry| dir_entry.path())
            .filter(|wallpaper_path| {
                wallpaper_path
                    .extension()
                    .map(|extension| ["jpg", "jpeg", "png"].contains(&extension.to_str().unwrap()))
                    .unwrap_or(false)
            })
            .collect();

        Ok(wallpaper_paths)
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
        Self {
            port: 46666,
            wallpaper_paths: vec![],
        }
    }
}

fn default_port() -> u16 {
    Config::default().port
}
