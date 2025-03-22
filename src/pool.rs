use std::path::PathBuf;

use crate::{Wallpaper, prelude::*};

#[derive(Clone, Debug)]
pub struct Pool {
    wallpapers: Vec<Wallpaper>,
}

impl Pool {
    pub fn new(wallpaper_names: Vec<String>, wallpaper_paths: &Vec<PathBuf>) -> Result<Self> {
        let mut wallpapers = vec![];
        for wallpaper_name in &wallpaper_names {
            let mut found = false;
            for wallpaper_path in wallpaper_paths {
                if wallpaper_path
                    .file_stem()
                    .expect("files should always have a file stem")
                    .to_string_lossy()
                    .to_string()
                    == *wallpaper_name
                {
                    wallpapers.push(Wallpaper::new(wallpaper_path)?);
                    found = true;
                }
            }
            if !found {
                error!("could not find wallpaper '{}'", wallpaper_name);
            }
        }

        Ok(Self { wallpapers })
    }

    pub fn wallpapers(&self) -> &Vec<Wallpaper> {
        &self.wallpapers
    }
}
