mod request_handling_logic;

use actix_web::{get, web, App, HttpServer, Responder, ResponseError};

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    request_handling_logic::inner_greet(&name).await
}


#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(greet)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}