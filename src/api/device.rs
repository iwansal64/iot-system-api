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

#[post("/device/get_controllable", data = "<body_data>")]
pub async fn get_controllable(db: &State<Database>, body_data: Json<DeviceConnectControllable>) -> status::Custom<String>  {
    let device_key = &body_data.device_key;
    let device_pass = &body_data.device_pass;
    let controllable_name = &body_data.controllable_name;

    // Get device data
    let device_data = db.verify_device_key_pass(device_key, device_pass).await;   
    
    let device_data = match device_data {
        Ok(res) => res,
        Err(err) => {
            return match err {
                ErrorType::DeviceNotFound(_) => status::Custom(http::Status::NotFound, format!("Device not found.")),
                _ => status::Custom(http::Status::InternalServerError, format!("There's an error."))
            };
        }
    };
    
    // Get controllable data
    let controllable_data = db.get_controllable(controllable_name).await;

    let controllable_data = match controllable_data {
        Ok(res) => res,
        Err(err) => {
            return match err {
                ErrorType::ControllableNotFound(_) => status::Custom(http::Status::NotFound, format!("Controllable not found.")),
                _ => status::Custom(http::Status::InternalServerError, format!("There's an error."))
            };
        }
    };

    // Get user data
    let user_data = db.get_user(&device_data.user_email).await;
    let user_data = match user_data {
        Ok(res) => res,
        Err(err) => {
            return match err {
                ErrorType::UserNotFound(_) => status::Custom(http::Status::NotFound, format!("User not found.")),
                _ => status::Custom(http::Status::InternalServerError, format!("There's an error.")),
            }
        }
    };
    
    status::Custom(http::Status::Ok, format!("{},{},{}", controllable_data.topic_name, user_data.mqtt_user, user_data.mqtt_pass))
}