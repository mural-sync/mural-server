use actix_web::{App, HttpResponse, HttpServer, Responder, get, http::StatusCode, web};

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

async fn not_found() -> impl Responder {
    HttpResponse::build(StatusCode::NOT_FOUND).body("Not Found")
}

#[get("/api/interval")]
async fn interval(state: web::Data<State>) -> impl Responder {
    HttpResponse::build(StatusCode::OK).body(state.interval().to_string())
}

#[get("/api/pool/{pool_name}/digest")]
async fn current_digest(path: web::Path<String>, state: web::Data<State>) -> impl Responder {
    let pool_name = path.into_inner();
    let wallpaper = state.current_wallpaper(&pool_name).unwrap();
    let digest = wallpaper.digest().clone();
    HttpResponse::build(StatusCode::OK).body(digest)
}

#[get("/api/pool/{pool_name}/wallpaper")]
async fn current_wallpaper(path: web::Path<String>, state: web::Data<State>) -> impl Responder {
    let pool_name = path.into_inner();
    let wallpaper = state.current_wallpaper(&pool_name).unwrap();
    let file_path = wallpaper.file_path().clone();
    let extension = file_path.extension().unwrap().to_string_lossy().to_string();
    let image_content = web::block(move || std::fs::read(file_path).unwrap())
        .await
        .unwrap();
    HttpResponse::build(StatusCode::OK)
        .content_type(format!("image/{}", extension))
        .body(image_content)
}

pub async fn run() -> Result<()> {
    env::load_dotenv()?;
    let config = Config::load()?;
    let state = State::new(&config)?;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .service(interval)
            .service(current_digest)
            .service(current_wallpaper)
            .default_service(web::route().to(not_found))
    })
    .bind(("0.0.0.0", config.port()))
    .unwrap()
    .run()
    .await
    .unwrap();

    Ok(())
}
