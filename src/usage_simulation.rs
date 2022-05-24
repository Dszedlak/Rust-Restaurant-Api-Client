use reqwest::blocking::Client;
use crate::{restaurant_api, models};
use rand::{seq::SliceRandom, Rng};
use chrono::prelude::*;
use std::io::Write;
use std::thread;
use std::time::Duration;

pub fn call_api(){
    let stdout_ref = &std::io::stdout();
    let mut lck = stdout_ref.lock();
    let client = Client::new();
    let mut rng = rand::thread_rng();

    //GetActiveSessions
    let active_tables = restaurant_api::get_active_sessions(&client).unwrap();
    let mut active_table_vec: Vec<u8> = Vec::new();
    let _ = writeln!(&mut lck, "Active Tables{:?}", active_tables);
    thread::sleep(Duration::from_millis(rng.gen_range(1..5)));

    let mut table_num: u8;
    for i in active_tables.into_iter()
    {   
        active_table_vec.push(i.table_nr);
    }

    table_num = rng.gen_range(1..101);
    while active_table_vec.contains(&table_num){
        table_num = rng.gen_range(1..101);
    }
    let _ = writeln!(&mut lck,"\n");
    let _ = writeln!(&mut lck,"Table to be added {:?}", table_num);
    
    let local: DateTime<Local> = Local::now();
    let session = models::TableSessionOut {
        customers: rng.gen_range(0..8),
        session_start: local.to_string(),
        active: true
    };

    //Add Session
    let add_session = restaurant_api::add_session(&client, table_num, session);
    let _ = writeln!(&mut lck,"Session Added: {:?}", add_session.unwrap());
    let _ = writeln!(&mut lck,"\n");
    thread::sleep(Duration::from_millis(rng.gen_range(1..5)));

    //GetSessions
    let sessions = restaurant_api::get_sessions(&client, table_num);
    let _ = writeln!(&mut lck,"Sessions for table {:?}: {:?}",table_num, sessions.unwrap());
    let _ = writeln!(&mut lck,"\n");
    thread::sleep(Duration::from_millis(rng.gen_range(1..5)));

    //GetActiveSession (Is session active?)
    let active_session = restaurant_api::get_active_session(&client, table_num).unwrap();
    let _ = writeln!(&mut lck,"Table {:?} has an active session? : {:?}",table_num, active_session);
    let _ = writeln!(&mut lck,"\n");
    thread::sleep(Duration::from_millis(rng.gen_range(1..5)));

    //GetItems
    let items = restaurant_api::get_items(&client).unwrap();
    let item_one: Vec<_> = items
    .choose_multiple(&mut rand::thread_rng(), 1)
    .collect();

    let mut item_two: Vec<_> = items
    .choose_multiple(&mut rand::thread_rng(), 1)
    .collect();

    while item_one[0].id == item_two[0].id {
        item_two = items
        .choose_multiple(&mut rand::thread_rng(), 1)
        .collect();
    };
    
    let mut random_item_index = rng.gen_range(1..10);

    while random_item_index == item_one[0].id || random_item_index == item_two[0].id 
    {
        random_item_index = rng.gen_range(1..10);
    }

    thread::sleep(Duration::from_millis(rng.gen_range(1..5)));

    //GetItem
    let item_three = restaurant_api::get_item(&client, random_item_index).unwrap();

    let mut item_ids: Vec<i64> = Vec::new();
    item_ids.push(item_one[0].id);
    item_ids.push(item_two[0].id);
    item_ids.push(item_three.id);

    let _ = writeln!(&mut lck,"(Get Items) Item 1: {:?}", item_ids[0]);
    let _ = writeln!(&mut lck,"(Get Items) Item 2: {:?}", item_ids[1]);
    let _ = writeln!(&mut lck,"(Get Item) Item 3: {:?}", item_ids[2]);
    let _ = writeln!(&mut lck,"\n");

    let order_item_one = models::OrderItem {
        item_id: item_ids[0],
        amount: rng.gen_range(1..8) 
    };

    let order_item_two = models::OrderItem {
        item_id: item_ids[1],
        amount: rng.gen_range(1..8) 
    };
    let order_item_three = models::OrderItem {
        item_id: item_ids[2],
        amount: rng.gen_range(1..8) 
    };

    let mut order_vec: Vec<models::OrderItem> = Vec::new();

    order_vec.push(order_item_one);
    order_vec.push(order_item_two);
    order_vec.push(order_item_three);

    let order = models::OrderOut {
        order_items: order_vec
    };

    //AddOrder
    let add_order = restaurant_api::add_order(&client, table_num, order);

    let _ = writeln!(&mut lck,"(Add order) Order added: {:?} for table {:?} ", add_order.unwrap(), table_num);   
    let _ = writeln!(&mut lck,"\n");
    
    thread::sleep(Duration::from_millis(rng.gen_range(1..5)));

    //GetOrders
    let orders = restaurant_api::get_orders(&client, table_num).unwrap();

    let order_id = orders[0].id;
    let item_id = orders[0].order_items[0].item_id;
    let _ = writeln!(&mut lck,"Table {:?}, Order {:?} to remove item {:?}", table_num, order_id, item_id);

    thread::sleep(Duration::from_millis(rng.gen_range(1..5)));

    //RemoveOrder
    let remove_item = restaurant_api::remove_item(&client, table_num, order_id, item_id);
    let _ = writeln!(&mut lck,"(Remove item) Removal {:?}", remove_item.unwrap()); 
    let _ = writeln!(&mut lck,"\n");

    thread::sleep(Duration::from_millis(rng.gen_range(1..5)));

    //GetOrder
    let updated_order = restaurant_api::get_order(&client, table_num, order_id);
    let _ = writeln!(&mut lck,"(Get order) Order after item {:?} was deleted {:?}", item_id, updated_order.unwrap()); 
    let _ = writeln!(&mut lck,"\n");

    thread::sleep(Duration::from_millis(rng.gen_range(1..5)));

    //RemoveOrder
    let remove_order = restaurant_api::remove_order(&client, table_num, order_id);
    let _ = writeln!(&mut lck,"(Remove order) Order to be Removed: {:?} for table {:?} ", remove_order.unwrap(), table_num);  
    let _ = writeln!(&mut lck,"\n");

    thread::sleep(Duration::from_millis(rng.gen_range(1..5)));

    //EndSession
    let end_session = restaurant_api::end_session(&client, table_num);
    let _ = writeln!(&mut lck,"(Remove session) Session to be Removed: {:?} for table {:?} ", end_session.unwrap(), table_num);  

    thread::sleep(Duration::from_millis(rng.gen_range(1..5)));
}