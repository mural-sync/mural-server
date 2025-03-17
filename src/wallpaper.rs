use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Wallpaper {
    pub file_path: PathBuf,
    pub digest: String,
}

impl Wallpaper {
    pub fn new<P: AsRef<Path>>(file_path: P) -> Result<Self, anyhow::Error> {
        let file_path = file_path.as_ref().to_path_buf();
        let digest = sha256::try_digest(&file_path)?;
        Ok(Self { file_path, digest })
    }
}
