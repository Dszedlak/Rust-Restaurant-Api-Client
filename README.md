Rust restaurant client
======
Client for the Restaurant backend API written in rust. 

***Please insure you have the latest version of Rust installed***

Libraries used: 

**[Reqwest](https://docs.rs/reqwest/latest/reqwest/)**

**[Serde](https://github.com/serde-rs/serde)**

**[Reqwest](https://docs.rs/reqwest/latest/reqwest/)**

**[Chrono](https://github.com/chronotope/chrono)**

**[rand](https://crates.io/crates/rand)**

## Getting started

1. Download the client (Either checkout the repository or download as a zip file).
2. Download the [server](https://github.com/Dszedlak/Rust-Restaurant-Api-Backend), and follow setup instructions. Do not proceed to the next step until the server is running.
3. Once downloaded, open a terminal in directory 'Rust_Restaurant_Client'. 
4. To run the client, simple type 'cargo run'. This will build the client, install any dependencies and run a simulated example of api calls to the server in 10 seperate threads.

### On start-up

**Output for the simulated api-calls will be displayed in the terminal. An example of expected output is below. Please note, that there should be no errors, and all 'Remove' calls should return 'success':**

(Get active sessions) Active Tables[]

Table to be added 32

(Add session) Session Added: TableSessionOut { customers: 4, session_start: "2022-05-24 23:47:18", active: true }


(Get sessions) Sessions for table 32:
[TableSession { id: 66, table_nr: 32, customers: 4, session_start: "2022-05-24 23:47:18", session_end: "", active: true }]

(Get active session) Table 32 has an active session? : 
TableSession { id: 66, table_nr: 32, customers: 4, session_start: "2022-05-24 23:47:18", session_end: "", active: true }


(Get Items) Item 1: 3
(Get Items) Item 2: 2
(Get Item) Item 3: 9


(Add order) Order added: OrderOut { order_items: 
[OrderItem { item_id: 3, amount: 4 }, OrderItem { item_id: 2, amount: 2 }, OrderItem { item_id: 9, amount: 2 }] } for table 32


(Get orders) Table 32, Order 64 to remove item 3
(Remove item) Removal "success"


(Get order) Order after item 3 was deleted Order 
{ id: 64, table_session_id: 66, timestamp: "2022-05-24 23:47:18",
order_items: [OrderItem { item_id: 2, amount: 2 }, OrderItem { item_id: 9, amount: 2 }] }


(Remove order) Order to be Removed: "success" for table 32 


(Remove session) Session to be Removed: "success" for table 32 
