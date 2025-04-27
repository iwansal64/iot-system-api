use rocket::{http, response::status, serde::json::Json, State};
use serde::{Deserialize, Serialize};

use crate::{db::Database, types::error::ErrorType};


#[derive(Serialize, Deserialize)]
pub struct DeviceInitialization {
    pub device_key: String,
    pub device_pass: String
}

#[derive(Serialize, Deserialize)]
pub struct DeviceConnectControllable {
    pub controllable_name: String,
    pub device_key: String,
    pub device_pass: String
}


#[post("/device/initialization", data = "<body_data>")]
pub async fn device_initialization(db: &State<Database>, body_data: Json<DeviceInitialization>) -> status::Custom<String> {
    let device_key = &body_data.device_key;
    let device_pass = &body_data.device_pass;

    match db.initialize_device(device_key, device_pass).await {
        Ok(_) => return status::Custom(http::Status::Ok, format!("OK")),
        Err(err) => {
            match err {
                ErrorType::DeviceNotFound(_) => {
                    return status::Custom(http::Status::NotFound, format!("NOT FOUND"));
                }
                ErrorType::UnknownError(message) => {
                    if let Some(msg) = message {
                        return status::Custom(http::Status::InternalServerError, msg);
                    }

                    return status::Custom(http::Status::InternalServerError, format!("ERROR"));
                },
                _ => {
                    return status::Custom(http::Status::InternalServerError, format!("ERROR"));
                }
            };
        }
    };
}

// #[post("/device/connect_controllable", data = "<body_data>")]
// pub async fn connect_controllable(body_data: Json<DeviceConnectControllable>) -> status::Custom<String>  {
//     status::Custom(http::Status::Ok, format!("Token"))
// }