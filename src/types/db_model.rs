use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

use crate::utils::{generate_long_token, generate_token};

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
    pub created_at: DateTime,
    pub user_email: String
}

impl Device {
    pub fn new(device_name: String, user_email: String) -> Self {
        Self {
            device_name,
            user_email,
            device_key: generate_long_token(),
            device_pass: generate_long_token(),
            id: ObjectId::new(),
            status: 0,
            created_at: DateTime::now(),
            last_online: None,
        }
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Controllable {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub controllable_name: String,
    pub device_id: ObjectId,
    pub created_at: DateTime,
    pub category: ControllableCategory,
    pub topic_name: String,
    pub user_email: String
}

impl Controllable {
    pub fn new(controllable_name: String, controllable_category: ControllableCategory, device_id: ObjectId, user_email: String) -> Self {
        Self {
            controllable_name,
            device_id,
            user_email,
            topic_name: generate_long_token(),
            id: ObjectId::new(),
            created_at: DateTime::now(),
            category: controllable_category,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ControllableCategory {
    Button,
    Slider,
    Switch,
    LED
}

impl ControllableCategory {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "Button" => Some(Self::Button),
            "Slider" => Some(Self::Slider),
            "Switch" => Some(Self::Switch),
            "LED" => Some(Self::LED),
            _ => None
        }
    }
}