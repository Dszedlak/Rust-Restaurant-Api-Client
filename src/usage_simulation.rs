use std::ptr::null;

use reqwest::{Client};
use crate::{restaurant_api::{self, get_active_session}, models::{self, TableSession}};
use rand::{seq::SliceRandom, Rng};
use chrono::prelude::*;


pub async fn call_api(){
    let client = Client::new();
    let items = restaurant_api::get_items(&client).await;
    let i = items.unwrap();

    let item_one: Vec<_> = i
    .choose_multiple(&mut rand::thread_rng(), 1)
    .collect();

    let item_two: Vec<_> = i
    .choose_multiple(&mut rand::thread_rng(), 1)
    .collect();
    
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

    
    println!("{:?}", item_one);
    println!("{:?}", item_two);
    //println!("{}", items.to_string());
} 