#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("failed to bind the server: {0}")]
    BindServer(std::io::Error),
    #[error("failed to run the server")]
    RunServer,

    #[error("no pool named '{0}'")]
    PoolNotFound(String),
    #[error("the pool '{0}' does not contain any wallpapers")]
    PoolEmpty(String),

    #[error("failed load a .env file: '{line_content}' on line {line_number} is invalid")]
    DotenvyParse {
        line_content: String,
        line_number: usize,
    },
    #[error("failed to read a .env file: {0}")]
    DotenvyIo(std::io::Error),

    #[error("failed to find a config directory")]
    ConfigHome,
    #[error("failed to parse the configuration: {0}")]
    ConfigParse(#[from] toml::de::Error),
    #[error("failed to read the configuration: {0}")]
    ConfigRead(std::io::Error),

    #[error("failed to list wallpapers: {0}")]
    WallpaperList(std::io::Error),
    #[error("failed to generate the digest for a wallpaper: {0}")]
    WallpaperDigest(String),

    #[error("{0}")]
    Custom(String),
}
