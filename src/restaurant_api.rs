use reqwest::{self, Client};
use crate::models::{self};
use std::{error::Error};
use serde_json::json;


const URL: &str = "http://localhost:8000/";

pub async fn get_item(client: &Client, item_num: u64) -> Result<models::Item, Box<dyn Error>> 
{
    let request_url = format!("{}items/{}", URL, item_num);
    let response = client.get(request_url).send().await?;
    let item: models::Item = response.json().await?;
    Ok(item)
}

pub async fn get_items(client: &Client) -> Result<Vec<models::Item>, Box<dyn Error>> 
{
    let request_url = format!("{}items/", URL);
    let response = client.get(request_url).send().await?;
    let items: Vec<models::Item> = response.json().await?;
    Ok(items)
}

pub async fn get_active_sessions(client: &Client) -> Result<Vec<models::TableSession>, Box<dyn Error>> 
{
    let request_url = format!("{}tables/", URL);
    let response = client.get(request_url).send().await?;
    let active_sessions: Vec<models::TableSession> = response.json().await?;
    Ok(active_sessions)
}

pub async fn get_sessions(client: &Client, table_nr: u8) -> Result<Vec<models::TableSession>, Box<dyn Error>>
{
    let request_url = format!("{}tables/{}", URL, table_nr);
    let response = client.get(request_url).send().await?;
    let sessions: Vec<models::TableSession> = response.json().await?;
    Ok(sessions)
}

pub async fn get_active_session(client: &Client, table_nr: u8) -> Result<models::TableSession, Box<dyn Error>>
{
    let request_url = format!("{}tables/{}/active", URL, table_nr);
    let response = client.get(request_url).send().await?;
    let sessions: models::TableSession = response.json().await?;
    Ok(sessions)
}//

pub async fn get_order(client: &Client, table_nr: u8 ,order_id: i64) -> Result<models::Order, Box<dyn Error>>
{
    let request_url = format!("{}tables/{}/orders/{})", URL, table_nr, order_id);
    let response = client.get(request_url).send().await?;
    let order: models::Order = response.json().await?;
    Ok(order)
}

pub async fn get_orders(client: &Client, table_nr: u8) -> Result<Vec<models::Order>, Box<dyn Error>>
{
    let request_url = format!("{}tables/{}/orders)", URL, table_nr);
    let response = client.get(request_url).send().await?;
    let orders: Vec<models::Order> = response.json().await?;
    Ok(orders)
}


//Post
pub async fn add_session(client: &Client, table_nr: u8, session: models::TableSessionOut) -> Result<models::TableSessionOut, Box<dyn Error>>
{
    let request_url = format!("{}tables/{}", URL, table_nr);
    let response = client.post(request_url).json(&session).send().await?;
    println!("{:?}", response);
    let added_session: models::TableSessionOut = response.json().await?;
    Ok(added_session)
}

pub async fn add_order(client: &Client, table_nr: u8, order: models::Order) -> Result<models::Order, Box<dyn Error>> 
{
    let request_url = format!("{}tables/{}/orders)", URL, table_nr);
    let response = client.post(request_url).body(serde_json::to_string(&order).unwrap()).send().await?;
    let add_order: models::Order = response.json().await?;
    Ok(add_order)
}

//Delete
pub async fn remove_order(client: &Client, table_nr: u8, order_id: i64) -> Result<String, Box<dyn Error>>
{
    let request_url = format!("{}tables/{}/orders/{})", URL, table_nr, order_id);
    let response = client.delete(request_url).send().await?;
    let removed: String = response.json().await?;
    Ok(removed)
}

pub async fn end_session(client: &Client, table_nr: u8) -> Result<String, Box<dyn Error>>
{
    let request_url = format!("{}tables/{})", URL, table_nr);
    let response = client.delete(request_url).send().await?;
    let removed: String = response.json().await?;
    Ok(removed)
}

pub async fn remove_item(client: &Client, table_nr: u8, order_id: i64, item_id: i64) -> Result<String, Box<dyn Error>>
{
    let request_url = format!("{}tables/{}/orders/{}/{})", URL, table_nr, order_id, item_id);
    let response = client.delete(request_url).send().await?;
    let removed: String = response.json().await?;
    Ok(removed)
}
