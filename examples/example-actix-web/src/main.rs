use std::time::Duration;

use actix_web::{
    error::ErrorNotFound,
    http::StatusCode,
    middleware,
    web::{self, Data},
    App, HttpRequest, HttpResponse, HttpServer, Result,
};
use include_dir_as_map::{include_dir_as_map, DirMap};

async fn index(_req: HttpRequest, assets: web::Data<DirMap>) -> Result<HttpResponse> {
    let asset = "index.html";
    let data = assets
        .get(asset)
        .ok_or_else(|| ErrorNotFound(StatusCode::NOT_FOUND))?
        .to_owned();

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(data))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let assets: DirMap = include_dir_as_map!("$CARGO_MANIFEST_DIR/build");
    let addr = "127.0.0.1:8080";
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Compress::default())
            .app_data(Data::new(assets.clone()))
            .route("/", web::get().to(index))
    })
    .keep_alive(Duration::from_secs(5))
    .bind(addr)
    .unwrap_or_else(|_| panic!("could not bind {}", addr))
    .run()
    .await
}
