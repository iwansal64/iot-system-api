#[macro_use] extern crate rocket;

pub mod api;
pub mod db;
pub mod types;
pub mod utils;
pub mod middlewares;

use api::{device::{device_initialization, get_controllable}, user::{confirm_registration, create_controllable, create_device, setup_registration, user_get, user_otp_login, user_otp_verify, user_password_login, user_registration}};
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
    let database: Database = Database::new(mongodb_uri.as_str(), "iotconnect_system_db", "user", "registration", "device", "controllable", "otp_login").await;

    rocket::build()
        .manage(database)
        .mount("/api/", 
            routes![
                /* User API */ 
                index, 
                user_registration,
                confirm_registration,
                setup_registration,
                user_password_login,
                user_get,
                create_device,
                user_otp_login,
                user_otp_verify,
                /* Device API */ 
                device_initialization,
                create_controllable,
                get_controllable
            ]
        )
}
