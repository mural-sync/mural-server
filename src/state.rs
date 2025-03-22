use std::collections::HashMap;

use rand::{SeedableRng, seq::IndexedRandom};

use crate::{Config, Pool, Wallpaper, prelude::*};

#[derive(Clone, Debug)]
pub struct State {
    interval: u64,
    pools: HashMap<String, Pool>,
}

impl State {
    pub fn new(config: &Config) -> Result<Self> {
        let mut pools = HashMap::new();
        for (pool_name, wallpaper_names) in config.pools() {
            let pool = Pool::new(wallpaper_names.to_vec(), config.wallpaper_paths())?;
            pools.insert(pool_name.to_string(), pool);
        }

        Ok(Self {
            interval: config.interval(),
            pools,
        })
    }

    pub fn current_wallpaper(&self, pool_name: &str) -> Result<&Wallpaper> {
        let pool = self.pools.get(pool_name).unwrap();
        let wallpapers = pool.wallpapers();

        let seed = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            / self.interval;
        let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(seed);

        Ok(wallpapers.choose(&mut rng).unwrap())
    }

    pub fn interval(&self) -> u64 {
        self.interval
    }
}
