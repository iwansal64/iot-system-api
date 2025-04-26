#[macro_use] extern crate rocket;

pub mod api;
pub mod db;
pub mod types;
pub mod utils;

use api::user::{confirm_registration, setup_registration, user_get, user_login, user_registration};
use db::Database;
use dotenvy::dotenv;
use std::env;

// GET route
#[get("/test")]
fn index() -> &'static str {
    "Hello from Rocket! 🚀"
}

#[launch]
async fn rocket() -> _ {
    dotenv().ok();
    
    let mongodb_uri = env::var("MONGO_DB_URI").expect("Please, Specify 'MONGO_DB_URI' in your '.env' file man.... :)\n");
    let database: Database = Database::new(mongodb_uri.as_str(), "iotconnect_system_db", "user", "registration").await;

    rocket::build()
        .manage(database)
        .mount("/api/", 
            routes![
                /* User API */ 
                index, 
                user_registration,
                confirm_registration,
                setup_registration,
                user_login,
                user_get
                /* Device API */ 
            ]
        )
}
