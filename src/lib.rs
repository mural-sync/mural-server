mod pool;
pub(crate) use pool::Pool;

mod state;
pub(crate) use state::State;

mod wallpaper;
pub(crate) use wallpaper::Wallpaper;

pub async fn run() -> Result<(), anyhow::Error> {
    let base_dirs = xdg::BaseDirectories::with_prefix("mural_server")?;

    let state = State::new(&base_dirs)?;
    dbg!(state);

    Ok(())
}
