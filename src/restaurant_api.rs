use reqwest::{self, Client};
use crate::models::{self, TableSession};
use std::{error::Error, fs::ReadDir};
use serde::Deserialize;

use url::{Url, ParseError};

pub fn build_full_url(client: Client, path: &str) -> Result<Url, ParseError> {
    const localhost: &'static str = "https://localhost:8000/";
    let base = Url::parse(localhost).expect("hardcoded URL is known to be valid");
    let joined = base.join(path)?;
    Ok(joined)
}

pub async fn get_items(client: Client) -> Result<Vec<models::Item>, Box<dyn Error>> 
{
    let request_url= format!("http://127.0.0.1:8000/items");  
    let response = reqwest::get(&request_url).await?;
    let items: Vec<models::Item> = response.json().await?;
    Ok(items)
}

pub async fn get_item(client: Client, item_num: u64) -> Result<models::Item, Box<dyn Error>> 
{
    let request_url= format!("http://127.0.0.1:8000/items/{}",item_num);  
    let response = reqwest::get(&request_url).await?;
    let item: models::Item = response.json().await?;
    Ok(item)
}

pub async fn get_active_sessions(client: Client) -> Result<Vec<models::TableSession>, Box<dyn Error>>
{
    Ok(())
}

pub async fn get_sessions(client: Client, table_nr: u8) -> Result<Vec<models::TableSession>, Box<dyn Error>>
{
    Ok(())
}

pub async fn get_active_session(client: Client, table_nr: u8) -> Result<models::TableSession, Box<dyn Error>>
{
    Ok(())
}//

pub async fn get_order(client: Client, table_nr: u8 ,order_id: i64) -> Result<models::Order, Box<dyn Error>>
{
    Ok(())
}

pub async fn get_orders(client: Client, table_nr: u8) -> Result<Vec<models::Order>, Box<dyn Error>>
{
    Ok(())
}


//Post
pub async fn add_table(client: Client, table_nr: u8, session: models::TableSession) -> Result<models::TableSession, Box<dyn Error>>
{
    //jSONify the tablesession struct
    Ok(())
}

pub async fn add_order(client: Client, table_nr: u8, order: models::Order) -> Result<models::Order, Box<dyn Error>> 
{
    Ok(())
}

//Delete

pub async fn remove_order(client: Client, table_nr: u8, order_id: i64) -> Result<String, Box<dyn Error>>
{
    Ok(())
}

pub async fn end_session(client: Client, table_nr: u8) -> Result<String, Box<dyn Error>>
{
    Ok(())
}

pub async fn remove_item(client: Client, table_nr: u8, order_id: i64, item_id: i64) -> Result<String, Box<dyn Error>>
{
    Ok(())
}
