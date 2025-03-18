use std::path::Path;

use rand::{SeedableRng, seq::IndexedRandom};

use crate::Wallpaper;

#[derive(Clone, Debug)]
pub struct Pool {
    wallpapers: Vec<Wallpaper>,
}

impl Pool {
    const TIME_SPAN: u64 = 3600;

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

    pub fn current_wallpaper(&self) -> Option<&Wallpaper> {
        if self.wallpapers.is_empty() {
            return None;
        }

        let seed = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("time went backwards")
            .as_secs()
            / Self::TIME_SPAN;
        let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(seed);
        self.wallpapers.choose(&mut rng)
    }
}
