mod config;
pub(crate) use config::Config;

mod env;

mod error;
pub(crate) use error::Error;

mod pool;
pub(crate) use pool::Pool;

pub(crate) mod prelude;
use prelude::*;

mod state;
pub(crate) use state::State;

mod wallpaper;
pub(crate) use wallpaper::Wallpaper;

pub fn run() -> Result<()> {
    env::load_dotenv()?;

    let config = Config::load()?;
    info!("using configuration {:?}", &config);

    let state = State::new(&config)?;
    info!("using state {:?}", &state);

    Ok(())
}
