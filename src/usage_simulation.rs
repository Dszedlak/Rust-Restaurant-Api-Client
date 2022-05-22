use reqwest::{Client};
use crate::{restaurant_api, models};
use rand::seq::SliceRandom;

pub async fn call_api(){
    let client = Client::new();
    let items = restaurant_api::get_items(client).await;
    let i = items.unwrap();
    let sample: Vec<_> = i
    .choose_multiple(&mut rand::thread_rng(), 1)
    .collect();
    println!("{:?}", sample);

    //println!("{}", items.to_string());
} 