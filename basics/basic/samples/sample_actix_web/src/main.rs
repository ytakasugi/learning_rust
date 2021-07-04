use actix_web::{get, App, HttpServer, Responder};
use actix_web::middleware::Logger;

#[get("/")]
async fn index() -> impl Responder {
    String::from("Hello World")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    // サーバを8080で起動。
    // Logger::default() をミドルウェアとして登録。
    HttpServer::new(|| App::new().wrap(Logger::default()).service(index))
        .bind("172.20.246.77:8080")?
        .run()
        .await
}