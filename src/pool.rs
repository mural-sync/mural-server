use std::path::Path;

use rand::{SeedableRng, seq::IndexedRandom};

use crate::Wallpaper;

#[derive(Clone, Debug)]
pub struct Pool {
    interval: u32,
    wallpapers: Vec<Wallpaper>,
}

impl Pool {
    pub fn new<P: AsRef<Path>>(pool_path: P, interval: u32) -> Result<Self, anyhow::Error> {
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

        Ok(Self {
            wallpapers,
            interval,
        })
    }

    pub fn current_wallpaper(&self) -> Option<&Wallpaper> {
        if self.wallpapers.is_empty() {
            return None;
        }

        let seed = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("time went backwards")
            .as_secs()
            / self.interval as u64;
        let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(seed);
        self.wallpapers.choose(&mut rng)
    }
}
