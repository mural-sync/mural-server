use std::collections::HashMap;

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
        let pool = self
            .pools
            .get(pool_name)
            .ok_or(Error::PoolNotFound(pool_name.to_string()))?;
        let wallpapers = pool.wallpapers();

        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("should always succeed, as time would have gone backwards otherwise")
            .as_secs();

        Ok(wallpapers
            .get(
                std::convert::TryInto::<usize>::try_into(
                    (timestamp / self.interval) % wallpapers.len() as u64,
                )
                .expect("would only fail when there are a huge number of wallpapers"),
            )
            .expect("index should always be in range"))
    }

    pub fn interval(&self) -> u64 {
        self.interval
    }

    pub fn pools(&self) -> &HashMap<String, Pool> {
        &self.pools
    }
}
