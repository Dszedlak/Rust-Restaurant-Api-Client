use std::ptr::null;

use reqwest::{Client};
use crate::{restaurant_api::{self, get_active_session}, models::{self, TableSession}};
use rand::{seq::SliceRandom, Rng};
use chrono::prelude::*;


pub async fn call_api(){
    let client = Client::new();
    
    let active_tables = restaurant_api::get_active_sessions(&client).await.unwrap();
    let mut active_table_vec: Vec<u8> = Vec::new();
    println!("Active Tables{:?}", active_tables);

    let mut rng = rand::thread_rng();
    let mut table_num: u8;
    
    for i in active_tables.into_iter()
    {   
        active_table_vec.push(i.table_nr);
    }

    table_num = rng.gen_range(0..101);
    while active_table_vec.contains(&table_num){
        table_num = rng.gen_range(0..101);
    }
    print!("\n");
    println!("Table to be added {:?}", table_num);
    
    let local: DateTime<Local> = Local::now();
    let session = models::TableSessionOut {
        customers: rng.gen_range(0..8),
        session_start: local.to_string(),
        active: true
    };

    let add_session = restaurant_api::add_session(&client, table_num, session).await;
    println!("Session Added: {:?}", add_session);
    print!("\n");

    let sessions = restaurant_api::get_sessions(&client, table_num).await.unwrap();
    println!("Sessions for table {:?}: {:?}",table_num, sessions);
    print!("\n");

    let active_session = restaurant_api::get_active_session(&client, table_num).await.unwrap();
    println!("Table {:?} has an active session? : {:?}",table_num, active_session);
    print!("\n");

    let items = restaurant_api::get_items(&client).await.unwrap();
    let item_one: Vec<_> = items
    .choose_multiple(&mut rand::thread_rng(), 1)
    .collect();

    let item_two: Vec<_> = items
    .choose_multiple(&mut rand::thread_rng(), 1)
    .collect();
    
    let random_item_index = rng.gen_range(0..10);
    let item_three = restaurant_api::get_item(&client, random_item_index).await.unwrap();

    let mut item_ids: Vec<i64> = Vec::new();
    item_ids.push(item_one[0].id);
    item_ids.push(item_two[0].id);
    item_ids.push(item_three.id);

    println!("(Get Items) Item 1: {:?}", item_ids[0]);
    println!("(Get Items) Item 2: {:?}", item_ids[1]);
    println!("(Get Item) Item 3: {:?}", item_ids[2]);

    let order_item_one = models::OrderItem {
        item_id: item_ids[0],
        amount: rng.gen_range(0..8) 
    };

    let order_item_two = models::OrderItem {
        item_id: item_ids[1],
        amount: rng.gen_range(0..8) 
    };
    let order_item_three = models::OrderItem {
        item_id: item_ids[2],
        amount: rng.gen_range(0..8) 
    };

    let mut order_vec: Vec<models::OrderItem> = Vec::new();

    order_vec.push(order_item_one);
    order_vec.push(order_item_two);
    order_vec.push(order_item_three);

    let order = models::OrderOut {
        order_items: order_vec
    };

    let add_order = restaurant_api::add_order(&client, table_num, order).await;

    println!("(Add order) Order added: {:?} for table {:?} ", add_order, table_num);   

    let orders = restaurant_api::get_orders(&client, table_num).await.unwrap();

    let order_id = orders[0].id;
    let item_id = orders[0].order_items[0].item_id;
    println!("Table {:?}, Order {:?} to remove item {:?}", table_num, order_id, item_id);
    
    let remove_item = restaurant_api::remove_item(&client, table_num, order_id, item_id).await;
    println!("Remove was {:?}", remove_item); 
} 