use std::{ffi::OsStr, path::PathBuf};

use rand::{SeedableRng, seq::SliceRandom};

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
                    == OsStr::new(wallpaper_name)
                {
                    wallpapers.push(Wallpaper::new(wallpaper_path)?);
                    found = true;
                }
            }
            if !found {
                error!("could not find wallpaper '{}'", wallpaper_name);
            }
        }

        let seed = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("should always succeed, as time would have gone backwards otherwise")
            .as_secs();
        let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(seed);
        wallpapers.shuffle(&mut rng);

        Ok(Self { wallpapers })
    }

    pub fn wallpapers(&self) -> &Vec<Wallpaper> {
        &self.wallpapers
    }
}
