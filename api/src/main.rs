mod handlers;

use actix_web::{web, App, HttpServer};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");

    // Start http server
    HttpServer::new(move || {
        App::new()
            .route("/game", web::get().to(handlers::game))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}