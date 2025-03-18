mod pool;
use actix_web::{
    App, HttpResponse, HttpServer, Responder, get,
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
    let pool = data.get_pool(pool_name).unwrap();
    let digest = pool.current_wallpaper().unwrap().digest().clone();
    HttpResponse::Ok().body(digest)
}

#[get("/pool/wallpaper")]
async fn pool_wallpaper(info: web::Query<Info>, data: web::Data<State>) -> impl Responder {
    let (extension, image_content) = web::block(move || {
        let pool_name = info.pool_name.clone();
        let pool = data.get_pool(&pool_name).unwrap();
        let file_path = pool.current_wallpaper().unwrap().file_path();
        (
            file_path.extension().unwrap().to_string_lossy().to_string(),
            std::fs::read(file_path).unwrap(),
        )
    })
    .await
    .unwrap();
    HttpResponse::Ok()
        .content_type(format!("image/{}", extension))
        .body(image_content)
}

pub async fn run() -> Result<(), anyhow::Error> {
    let base_dirs = xdg::BaseDirectories::with_prefix("mural_server")?;

    let state = State::new(&base_dirs)?;

    HttpServer::new(move || {
        App::new()
            .service(pool_digest)
            .service(pool_wallpaper)
            .app_data(web::Data::new(state.clone()))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;

    Ok(())
}
