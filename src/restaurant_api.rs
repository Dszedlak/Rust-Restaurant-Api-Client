use reqwest::blocking::Client;
use crate::models;
use std::{error::Error};

const URL: &str = "http://localhost:8000/";

pub fn get_item(client: &Client, item_num: i64) -> Result<models::Item, Box<dyn Error>> 
{
    let request_url = format!("{}items/{}", URL, item_num);
    let response = client.get(request_url).send();
    let item: models::Item = response?.json()?;
    Ok(item)
}

pub fn get_items(client: &Client) -> Result<Vec<models::Item>, Box<dyn Error>> 
{
    let request_url = format!("{}items/", URL);
    let response = client.get(request_url).send();
    let items: Vec<models::Item> = response?.json()?;
    Ok(items)
}

pub fn get_active_sessions(client: &Client) -> Result<Vec<models::TableSession>, Box<dyn Error>> 
{
    let request_url = format!("{}tables/", URL);
    let response = client.get(request_url).send();
    let active_sessions: Vec<models::TableSession> = response?.json()?;
    Ok(active_sessions)
}

pub fn get_sessions(client: &Client, table_nr: u8) -> Result<Vec<models::TableSession>, Box<dyn Error>>
{
    let request_url = format!("{}tables/{}", URL, table_nr);
    let response = client.get(request_url).send();
    let sessions: Vec<models::TableSession> = response?.json()?;
    Ok(sessions)
}

pub fn get_active_session(client: &Client, table_nr: u8) -> Result<models::TableSession, Box<dyn Error>>
{
    let request_url = format!("{}tables/{}/active", URL, table_nr);
    let response = client.get(request_url).send();
    let sessions: models::TableSession = response?.json()?;
    Ok(sessions)
}//

pub fn get_order(client: &Client, table_nr: u8 ,order_id: i64) -> Result<models::Order, Box<dyn Error>>
{
    let request_url = format!("{}tables/{}/orders/{}", URL, table_nr, order_id);
    let response = client.get(request_url).send();
    let order: models::Order = response?.json()?;
    Ok(order)
}

pub fn get_orders(client: &Client, table_nr: u8) -> Result<Vec<models::Order>, Box<dyn Error>>
{
    let request_url = format!("{}tables/{}/orders", URL, table_nr);
    let response = client.get(request_url).send();
    let orders: Vec<models::Order> = response?.json()?;
    Ok(orders)
}


//Post
pub fn add_session(client: &Client, table_nr: u8, session: models::TableSessionOut) -> Result<models::TableSessionOut, Box<dyn Error>>
{
    let request_url = format!("{}tables/{}", URL, table_nr);
    let response = client.post(request_url).json(&session).send();
    let added_session: models::TableSessionOut = response?.json()?;
    Ok(added_session)
}

pub fn add_order(client: &Client, table_nr: u8, order: models::OrderOut) -> Result<models::OrderOut, Box<dyn Error>> 
{
    let request_url = format!("{}tables/{}/orders", URL, table_nr);
    let response = client.post(request_url).json(&order).send();
    let add_order: models::OrderOut = response?.json()?;
    Ok(add_order)
}

//Delete
pub fn remove_order(client: &Client, table_nr: u8, order_id: i64) -> Result<String, Box<dyn Error>>
{
    let request_url = format!("{}tables/{}/orders/{}", URL, table_nr, order_id);
    let response = client.delete(request_url).send();
    let removed: String = response?.json()?;
    Ok(removed)
}

pub fn end_session(client: &Client, table_nr: u8) -> Result<String, Box<dyn Error>>
{
    let request_url = format!("{}tables/{}", URL, table_nr);
    let response = client.delete(request_url).send();
    let removed: String = response?.json()?;
    Ok(removed)
}

pub fn remove_item(client: &Client, table_nr: u8, order_id: i64, item_id: i64) -> Result<String, Box<dyn Error>>
{
    let request_url = format!("{}tables/{}/orders/{}/{}", URL, table_nr, order_id, item_id);
    let response = client.delete(request_url).send();
    let removed: String = response?.json()?;
    Ok(removed)
}