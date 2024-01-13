use std::time::Duration;
use actix_web::Responder;
use tokio::time::sleep;
use tracing::info;

// this function simulates validating the request
// we instrument a function which simply
// means we're creating a new span that will be entered
// before we enter this function and will be closed after
// we enter this function
#[tracing::instrument]
async fn validate_request() {
    info!("Validating request");
    sleep(Duration::from_millis(50)).await;
    info!("Successfully validated request");
}

// this function simulates storing to db
#[tracing::instrument]
async fn store_to_db(_name: &str) {
    info!("Storing to DB");
    sleep(Duration::from_millis(150)).await;
    info!("Successfully stored to DB");
}

// this is the function that does all the logic for the request handling
#[tracing::instrument]
pub(crate) async fn inner_greet(name: &str) -> impl Responder {
    info!("Handling request");
    validate_request().await;
    store_to_db(name).await;
    info!("Successfully handled the request");
    format!("Hello {name}, you have beed stored in the DB!")
}