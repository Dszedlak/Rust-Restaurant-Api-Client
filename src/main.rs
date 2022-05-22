mod restaurant_api;
mod models;
mod usage_simulation;
use reqwest::{self};
use reqwest::Error;


#[tokio::main]
async fn main() -> Result<(), Error> {
    usage_simulation::call_api().await;
    Ok(())
}