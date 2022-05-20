mod restaurant_api;
mod models;

use serde::Deserialize;
use reqwest::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    
    let request_url= format!("http://127.0.0.1:8000/items");
   
    let response = reqwest::get(&request_url).await?;

    let items: Vec<models::Item> = response.json().await?;

    println!("{}", "ooger");
    println!("{:?}", items);
    Ok(())

}