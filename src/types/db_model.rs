use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

use crate::utils::generate_token;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub username: String,
    pub email: String,
    pub password: String
}

impl User {
    pub fn new(username: String, email: String, password: String) -> Self {
        Self {
            username,
            email,
            password,
            id: ObjectId::new()
        }
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct RegistrationTable {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub confirmation_token: String,
    pub setup_token: String,
    pub email: String,
    pub created_at: DateTime,
    pub confirmed: bool
}

impl RegistrationTable {
    pub fn new(email: String) -> Self {
        Self {
            email,
            id: ObjectId::new(),
            confirmation_token: generate_token(),
            confirmed: false,
            created_at: DateTime::now(),
            setup_token: generate_token()
        }
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Device {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub device_name: String,
    pub status: i32,
    pub device_key: String,
    pub device_pass: String,
    pub last_online: Option<DateTime>,
    pub created_at: DateTime
}

impl Device {
    pub fn new(device_name: String) -> Self {
        Self {
            device_name,
            device_key: generate_token(),
            device_pass: generate_token(),
            id: ObjectId::new(),
            status: 0,
            created_at: DateTime::now(),
            last_online: None
        }
    }
}