use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: ObjectId,
    pub username: String,
    pub email: String,
    pub password: String
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