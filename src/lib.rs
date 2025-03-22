mod config;
use actix_web::{App, HttpResponse, HttpServer, Responder, get, http::StatusCode, web};
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

async fn interval(data: web::Data<State>) -> impl Responder {
    HttpResponse::build(StatusCode::OK).body(data.interval().to_string())
}

pub async fn run() -> Result<()> {
    env::load_dotenv()?;
    let config = Config::load()?;
    let state = State::new(&config)?;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .service(web::scope("/api").route("interval", web::get().to(interval)))
            .default_service(web::route().to(not_found))
    })
    .bind(("0.0.0.0", config.port()))
    .unwrap()
    .run()
    .await
    .unwrap();

    Ok(())
}
