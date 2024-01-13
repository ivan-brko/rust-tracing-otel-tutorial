mod request_handling_logic;
mod init;
//this is new
mod custom_span_processor;

use actix_web::{get, web, App, HttpServer, Responder};
use tracing::info;
use tracing_actix_web::TracingLogger;

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    request_handling_logic::inner_greet(&name).await
}


#[tokio::main]
async fn main() -> std::io::Result<()> {
    //make sure to add it before starting the server
    //or we will not collect any logs in the application

    //we now have to await this
    init::initialize_tracing_subscriber().await;

    info!("Starting the web server");

    HttpServer::new(|| {
        App::new()
            .wrap(TracingLogger::default())
            .service(greet)
    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await?;

    info!("Shutting down web server");
    Ok(())
}