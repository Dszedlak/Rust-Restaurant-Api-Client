mod restaurant_api;
mod models;
mod usage_simulation;
use reqwest::{self};
use reqwest::Error;
use std::thread;
use futures::executor::block_on;


#[tokio::main]
async fn main() -> Result<(), Error> {
    // thread::spawn( move || block_on(usage_simulation::call_api()));
    // thread::spawn( move || block_on(usage_simulation::call_api()));
    // thread::spawn( move || block_on(usage_simulation::call_api()));
    // thread::spawn( move || block_on(usage_simulation::call_api()));
    // thread::spawn( move || block_on(usage_simulation::call_api()));
    // thread::spawn( move || block_on(usage_simulation::call_api()));
    // thread::spawn( move || block_on(usage_simulation::call_api()));
    // thread::spawn( move || block_on(usage_simulation::call_api()));
    // thread::spawn( move || block_on(usage_simulation::call_api()));
    // thread::spawn( move || block_on(usage_simulation::call_api()));
    usage_simulation::call_api().await;
    Ok(())
}