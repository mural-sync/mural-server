use std::collections::HashMap;

use crate::{Config, Pool, prelude::*};

#[derive(Clone, Debug)]
pub struct State {
    pools: HashMap<String, Pool>,
}

impl State {
    pub fn new(config: &Config) -> Result<Self> {
        let mut pools = HashMap::new();
        for (pool_name, wallpaper_names) in config.pools() {
            let pool = Pool::new(wallpaper_names.to_vec(), config.wallpaper_paths())?;
            pools.insert(pool_name.to_string(), pool);
        }

        Ok(Self { pools })
    }

    pub fn pools(&self) -> &HashMap<String, Pool> {
        &self.pools
    }
}
