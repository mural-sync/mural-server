mod pool;
use actix_web::{
    App, HttpResponse, HttpServer, Responder, get,
    http::StatusCode,
    web::{self},
};
pub(crate) use pool::Pool;

mod state;
pub(crate) use state::State;

mod wallpaper;
pub(crate) use wallpaper::Wallpaper;

#[derive(serde::Deserialize)]
struct Info {
    pool_name: String,
}

#[get("/pool/digest")]
async fn pool_digest(info: web::Query<Info>, data: web::Data<State>) -> impl Responder {
    let pool_name = &info.pool_name;
    let pool = match data.get_pool(pool_name) {
        Some(pool) => pool,
        None => return HttpResponse::build(StatusCode::NOT_FOUND).body("no such pool"),
    };
    let digest = match pool.current_wallpaper() {
        Some(current_wallpaper) => current_wallpaper.digest().clone(),
        None => {
            tracing::info!("failed to get current wallpaper");
            return HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).finish();
        }
    };
    HttpResponse::build(StatusCode::OK).body(digest)
}

#[get("/pool/wallpaper")]
async fn pool_wallpaper(info: web::Query<Info>, data: web::Data<State>) -> impl Responder {
    let pool_name = &info.pool_name;
    let pool = match data.get_pool(pool_name) {
        Some(pool) => pool,
        None => return HttpResponse::build(StatusCode::NOT_FOUND).body("no such pool"),
    };
    let file_path = match pool.current_wallpaper() {
        Some(current_wallpaper) => current_wallpaper.file_path().clone(),
        None => {
            tracing::info!("failed to get current wallpaper");
            return HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).finish();
        }
    };
    let extension = file_path.extension().unwrap().to_string_lossy().to_string();
    let image_content = web::block(move || std::fs::read(&file_path).unwrap())
        .await
        .unwrap();
    HttpResponse::build(StatusCode::OK)
        .content_type(format!("image/{}", extension))
        .body(image_content)
}

pub async fn run() -> Result<(), anyhow::Error> {
    let port: u16 = std::env::var("MURAL_SERVER_PORT")
        .unwrap_or("46666".to_string())
        .parse::<u16>()?;
    let interval: u32 = std::env::var("MURAL_SERVER_INTERVAL")
        .unwrap_or("600".to_string())
        .parse::<u32>()?;

    let base_dirs = xdg::BaseDirectories::with_prefix("mural_server")?;
    let config_home = base_dirs.get_config_home().join("pools");
    std::fs::create_dir_all(&config_home)?;

    let state = State::new(&base_dirs, interval)?;

    HttpServer::new(move || {
        App::new()
            .service(pool_digest)
            .service(pool_wallpaper)
            .app_data(web::Data::new(state.clone()))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await?;

    Ok(())
}
