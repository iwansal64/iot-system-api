use arrayvec::ArrayString;
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use rand::seq::IndexedRandom;
use regex::Regex;
use rocket::http::CookieJar;
use std::env;

pub fn is_valid_email(email: &str) -> bool {
    let re = Regex::new(r"^[\w\.-]+@[\w\.-]+\.\w+$").unwrap();
    re.is_match(email)
}

pub fn generate_token() -> String {
    let mut rng = rand::rng();
    let characters_combinations = ('a'..='z').chain('A'..'Z').chain('0'..'9').collect::<Vec<char>>();
    let mut generated_confirmation_token: ArrayString<5> = ArrayString::new();
    for _ in 0..5 {
        generated_confirmation_token.push(characters_combinations.choose(&mut rng).unwrap().clone());
    }

    generated_confirmation_token.to_string()
}

use serde::{Deserialize, Serialize};

use crate::types::error::ErrorType;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub fn create_user_token(user_email: &str) -> String {
    let secret: String = env::var("JWT_TOKEN").expect("Please, set up 'JWT_TOKEN' in your .env");
    let expiration: usize = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims: Claims = Claims {
        sub: user_email.to_owned(),
        exp: expiration,
    };

    jsonwebtoken::encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_bytes()))
        .expect("Token creation failed")
}

pub fn verify_user_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let secret: String = env::var("JWT_TOKEN").expect("Please, set up 'JWT_TOKEN' in your .env");
    let token_data = jsonwebtoken::decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )?;
    Ok(token_data.claims)
}

pub fn verify_user_token_from_cookie(cookies: &CookieJar<'_>) -> Result<String, ErrorType> {
    match cookies.get("user_token") {
        Some(user_token) => {
            match verify_user_token(user_token.value()) {
                Ok(data) => {
                    Ok(data.sub)
                },
                Err(err) => {
                    println!("Error: {}", err.to_string());
                    return Err(ErrorType::Unauthorized(None));
                    // return status::Custom(http::Status::Unauthorized, Json(ResponseBody { message: format!("Unauthorized."), success: false, data: None }));
                }
            }
        },
        None => {
            return Err(ErrorType::Unauthorized(None));
            // return status::Custom(http::Status::Unauthorized, Json(ResponseBody { message: format!("Unauthorized Attempt!"), success: false, data: None }));
        }
    }
}