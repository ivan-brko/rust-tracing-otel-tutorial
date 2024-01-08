use std::time::Duration;
use actix_web::Responder;
use tokio::time::sleep;

async fn validate_request() {
    sleep(Duration::from_millis(50)).await;
}

async fn store_to_db(_name: &str) {
    sleep(Duration::from_millis(150)).await;
}

pub(crate) async fn inner_greet(name: &str) -> impl Responder {
    validate_request().await;
    store_to_db(name).await;
    format!("Hello {name}, you have beed stored in the DB!")
}