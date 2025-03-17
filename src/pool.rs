use std::path::Path;

use crate::Wallpaper;

#[derive(Debug)]
pub struct Pool {
    pub wallpapers: Vec<Wallpaper>,
}

impl Pool {
    pub fn new<P: AsRef<Path>>(pool_path: P) -> Result<Self, anyhow::Error> {
        let pool_path = pool_path.as_ref().to_path_buf();

        let mut wallpapers = vec![];
        for file_path in pool_path.read_dir()? {
            let file_path = file_path?.path();
            if file_path.is_dir() {
                continue;
            }

            let wallpaper = Wallpaper::new(file_path)?;
            wallpapers.push(wallpaper);
        }

        Ok(Self { wallpapers })
    }
}
