mod config;
use actix_web::{App, HttpServer, web};
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

pub async fn run() -> Result<()> {
    env::load_dotenv()?;
    let config = Config::load()?;
    let state = State::new(&config)?;

    HttpServer::new(move || App::new().app_data(web::Data::new(state.clone())))
        .bind(("0.0.0.0", config.port()))
        .unwrap()
        .run()
        .await
        .unwrap();

    Ok(())
}
