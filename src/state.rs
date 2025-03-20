use std::collections::HashMap;

use crate::Pool;

#[derive(Clone, Debug)]
pub struct State {
    pools: HashMap<String, Pool>,
}

impl State {
    pub fn new(base_dirs: &xdg::BaseDirectories, interval: u32) -> Result<Self, anyhow::Error> {
        let pools_path = base_dirs.get_config_home().join("pools");

        let mut pools = HashMap::new();
        for pool_path in pools_path.read_dir()? {
            let pool_path = pool_path?.path();
            if pool_path.is_file() {
                continue;
            }

            let pool_name = pool_path.file_name().unwrap().to_string_lossy().to_string();
            let pool = Pool::new(pool_path, interval)?;

            pools.insert(pool_name, pool);
        }

        Ok(Self { pools })
    }

    pub fn get_pool(&self, pool_name: &str) -> Option<&Pool> {
        self.pools.get(pool_name)
    }
}
