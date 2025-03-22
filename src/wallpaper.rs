use std::path::{Path, PathBuf};

use crate::prelude::*;

#[derive(Clone, Debug)]
pub struct Wallpaper {
    digest: String,
    file_path: PathBuf,
}

impl Wallpaper {
    pub fn new<P: AsRef<Path>>(file_path: P) -> Result<Self> {
        let file_path = file_path.as_ref().to_path_buf();
        let digest = sha256::try_digest(&file_path).unwrap();
        Ok(Self { digest, file_path })
    }

    pub fn digest(&self) -> &String {
        &self.digest
    }

    pub fn file_path(&self) -> &PathBuf {
        &self.file_path
    }
}
