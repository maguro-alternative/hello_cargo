use actix_web::{get, App, HttpServer, Responder};

#[get("/")]
async fn index() -> impl Responder {
    // 文字列をそのまま返す
    "Hello World"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // HTTPサーバを8080で起動
    HttpServer::new(|| App::new().service(index))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}