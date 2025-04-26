use mongodb::{bson::{doc, oid::ObjectId, DateTime}, options::ClientOptions, Client, Collection};

use crate::{types::error::ErrorType, types::db_model::{RegistrationTable, User}};

pub struct Database {
    user: Collection<User>,
    registration: Collection<RegistrationTable>,
}

impl Database {
    pub async fn new(mongodb_uri: &str, database_name: &str, user_collection_name: &str, reg_table_collection_name: &str) -> Self {
        let options: ClientOptions = ClientOptions::parse(mongodb_uri).await.unwrap();
        let client: Client = Client::with_options(options).unwrap();
        let db: mongodb::Database = client.database(database_name);
        let user_col: Collection<User> = db.collection::<User>(user_collection_name);
        let registration_col: Collection<RegistrationTable> = db.collection::<RegistrationTable>(reg_table_collection_name);

        Self {
            user: user_col,
            registration: registration_col
        }
    }

    pub async fn get_user(&self, email: &str) -> Result<User, ErrorType>{
        //? Create query to find the user based on it's ID
        let query_result = self.user.find_one(doc! {
            "email": email
        }).await;

        //? Get the result while handling both 'query error' and 'user not found'.
        match query_result {
            Ok(user_data_exist) => {
                match user_data_exist {
                    Some(user_data) => Ok(user_data),
                    None => Err(ErrorType::UserNotFound(Some(format!("There's no user with ID: {}", email))))
                }
            },
            Err(err) => Err(ErrorType::UnknownError(Some(err.to_string())))
        }
    }

    pub async fn insert_registration(&self, confirmation_token: &String, setup_token: &String, email: &String) -> Result<String, ErrorType> {
        println!("[Insert Registration] Email: {}", email);

        //? Verify there's no duplicates
        let duplicated_data: Option<User> = match self.user.find_one(doc! {
            "email": email.clone()
        }).await {
            Ok(user) => user,
            Err(err) => {
                println!("There's an error when trying to find user for duplication check. Error: {}", err.to_string());
                return Err(ErrorType::UnknownError(Some(err.to_string())));
            }
        };

        if duplicated_data.is_some() {
            return Err(ErrorType::DuplicatesFound(None));
        }

        
        //? Prepare the required data value
        let id: ObjectId = ObjectId::new();
        let created_at: DateTime = DateTime::now();

        let registration_entry = RegistrationTable {
            id,
            confirmation_token: confirmation_token.clone(),
            setup_token: setup_token.clone(),
            email: email.clone(),
            created_at,
            confirmed: false
        };

        //? Create a query to insert the new registration data
        let query_result: Result<mongodb::results::InsertOneResult, mongodb::error::Error> = self.registration.insert_one(registration_entry).await;


        //? Check if there's error. if not, send the ID
        match query_result {
            Ok(_) => Ok(id.to_string()),
            Err(error) => Err(ErrorType::UnknownError(Some(error.to_string())))
        }
    }

    pub async fn get_confirmation_data(&self, target_id: &ObjectId, confirmation_token: &String) -> Result<RegistrationTable, ErrorType> {
        //? Create query to find the confirmation data based on it's ID and Confirmation Token
        println!("[Get Confirmation Data] Target ID: {}", target_id);
        let query_result = self.registration.find_one(doc! {
            "_id": target_id.clone(),
            "confirmation_token": confirmation_token.clone()
        }).await;

        //? Get the result while handling both 'query error' and 'confirmation data not found'.
        let registration_data: RegistrationTable = match query_result {
            Ok(user_data_exist) => {
                match user_data_exist {
                    Some(user_data) => user_data,
                    None => {
                        return Err(ErrorType::Unauthorized(None));
                    }
                }
            },
            Err(err) => {
                return Err(ErrorType::UnknownError(Some(err.to_string())));
            }
        };

        //? Update verification data
        match self.registration.update_one(doc! {
            "_id": target_id.clone(),
            "confirmation_token": confirmation_token.clone()
        }, doc! {
            "$set": {
                "confirmed": true
            }
        }).await {
            Ok(_) => Ok(registration_data),
            Err(err) => {
                println!("There's an error when trying to update registration data. Error: {}", err);
                return Err(ErrorType::UnknownError(Some(err.to_string())));
            }
        }
    }

    pub async fn setup_account(&self, target_id: &ObjectId, setup_token: &String, username: &String, password: &String) -> Result<RegistrationTable, ErrorType> {
        //? Create query to find the confirmation data based on it's ID and Confirmation Token
        println!("[Setup Account] Target ID: {}", target_id);
        let query_result = self.registration.find_one(doc! {
            "_id": target_id.clone(),
            "setup_token": setup_token.clone()
        }).await;


        //? Get the result while handling both 'query error' and 'confirmation data not found'.
        let registration_data: RegistrationTable = match query_result {
            Ok(user_data_exist) => {
                match user_data_exist {
                    Some(user_data) => user_data,
                    None => {
                        return Err(ErrorType::Unauthorized(None));
                    }
                }
            },
            Err(err) => {
                return Err(ErrorType::UnknownError(Some(err.to_string())));
            }
        };


        //? Verify there's no duplicates
        let duplicated_data: Option<User> = match self.user.find_one(doc! {
            "username": username.clone(),
            "email": registration_data.email.clone()
        }).await {
            Ok(user) => user,
            Err(err) => {
                println!("There's an error when trying to find user for duplication check. Error: {}", err.to_string());
                return Err(ErrorType::UnknownError(Some(err.to_string())));
            }
        };

        match duplicated_data {
            Some(_) => {
                return Err(ErrorType::Unauthorized(None))
            },
            None => ()
        };

        
        //? Create user account
        let user_creation_result = self.user.insert_one(User {
            id: ObjectId::new(),
            username: username.clone(),
            password: password.clone(),
            email: registration_data.email.clone(),
        }).await;

        match user_creation_result {
            Ok(_) => Ok(registration_data),
            Err(err) => Err(ErrorType::UnknownError(Some(err.to_string())))
        }
    }

    pub async fn verify_login(&self, username: &String, password: &String) -> Result<User, ErrorType> {
        println!("Username: {}", username);

        //? Create query to get the user data based on it's username
        let query_result = self.user.find_one(doc! {
            "username": username.clone()
        }).await;


        //? Get the user data and checks the password
        match query_result {
            Ok(res) => {
                match res {
                    Some(user_data) => {
                        if user_data.password.eq(password) {
                            return Ok(user_data);
                        }
                        
                        Err(ErrorType::Unauthorized(None))
                    },
                    None => Err(ErrorType::UserNotFound(None))
                }
            },
            Err(err) => Err(ErrorType::UnknownError(Some(err.to_string())))
        }
    }
}