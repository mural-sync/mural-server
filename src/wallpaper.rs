use std::path::{Path, PathBuf};

#[derive(Clone, Debug)]
pub struct Wallpaper {
    file_path: PathBuf,
    digest: String,
}

impl Wallpaper {
    pub fn new<P: AsRef<Path>>(file_path: P) -> Result<Self, anyhow::Error> {
        let file_path = file_path.as_ref().to_path_buf();
        let digest = sha256::try_digest(&file_path)?;
        Ok(Self { file_path, digest })
    }

    pub fn file_path(&self) -> &PathBuf {
        &self.file_path
    }

    pub fn digest(&self) -> &String {
        &self.digest
    }
}
