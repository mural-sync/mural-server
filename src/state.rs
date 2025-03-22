use std::collections::HashMap;

use crate::{Config, Pool, prelude::*};

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

    pub fn interval(&self) -> u64 {
        self.interval
    }

    pub fn pools(&self) -> &HashMap<String, Pool> {
        &self.pools
    }
}
