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
    let wallpaper = match state.current_wallpaper(&pool_name) {
        Ok(wallpaper) => wallpaper,
        Err(Error::PoolNotFound(pool_name)) => {
            return HttpResponse::build(StatusCode::NOT_FOUND)
                .body(format!("there is no pool named '{}'", pool_name));
        }
        Err(Error::PoolEmpty(pool_name)) => {
            error!("the current wallpaper of a pool was requested, but it is empty");
            return HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).body(format!(
                "the pool '{}' does not contain any wallpapers",
                pool_name
            ));
        }
        Err(_) => todo!(
            "current_wallpaper returned an error that was not expected; you should handle the new error here"
        ),
    };
    let digest = wallpaper.digest().clone();
    HttpResponse::build(StatusCode::OK).body(digest)
}

#[get("/api/pool/{pool_name}/wallpaper")]
async fn current_wallpaper(path: web::Path<String>, state: web::Data<State>) -> impl Responder {
    let pool_name = path.into_inner();
    let wallpaper = match state.current_wallpaper(&pool_name) {
        Ok(wallpaper) => wallpaper,
        Err(Error::PoolNotFound(pool_name)) => {
            return HttpResponse::build(StatusCode::NOT_FOUND)
                .body(format!("there is no pool named '{}'", pool_name));
        }
        Err(Error::PoolEmpty(pool_name)) => {
            error!("the current wallpaper of a pool was requested, but it is empty");
            return HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).body(format!(
                "the pool '{}' does not contain any wallpapers",
                pool_name
            ));
        }
        Err(_) => todo!(
            "current_wallpaper returned an error that was not expected; you should handle the new error here"
        ),
    };
    let file_path = wallpaper.file_path().clone();
    let extension = file_path
        .extension()
        .expect("files with no extension should be filtered out from the pool")
        .to_string_lossy()
        .to_string();
    let image_content = match web::block(move || std::fs::read(file_path)).await {
        Ok(Ok(image_content)) => image_content,
        Ok(Err(e)) => {
            error!("failed to read the wallpaper file: {}", e);
            return HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
                .body("failed to read the wallpaper file");
        }
        Err(e) => {
            error!("blocking error: {}", e);
            return HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
                .body("internal server error");
        }
    };
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
    .map_err(|e| Error::BindServer(e))?
    .run()
    .await
    .map_err(|_| Error::RunServer)?;

    Ok(())
}
