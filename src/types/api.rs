use crate::types::db_model::User;
use serde::Serialize;

use super::db_model::{Controllable, Device};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ResponseBody {
    pub message: String,
    pub success: bool,
    pub data: Option<ResponseBodyType>
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum ResponseBodyType {
    UserRegistration {
        id: String
    },
    UserVerify {
        token: String,
        id: String
    },
    UserSetup {
        user_data: User
    },
    UserLogin {
        user_data: User
    },
    UserGet {
        user_data: User
    },
    CreateDevice {
        device_data: Device
    },
    CreateControllable {
        controllable_data: Controllable
    }
}