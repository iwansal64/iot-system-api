use mongodb::{bson::{doc, oid::ObjectId, DateTime}, options::ClientOptions, Client, Collection};

use crate::types::{db_model::{Controllable, ControllableCategory, Device, RegistrationTable, User}, error::ErrorType};

pub struct Database {
    user: Collection<User>,
    registration: Collection<RegistrationTable>,
    device: Collection<Device>,
    controllable: Collection<Controllable>
}

impl Database {
    pub async fn new(mongodb_uri: &str, database_name: &str, user_collection_name: &str, reg_table_collection_name: &str, device_collection_name: &str, controllable_collection_name: &str) -> Self {
        let options: ClientOptions = ClientOptions::parse(mongodb_uri).await.unwrap();
        let client: Client = Client::with_options(options).unwrap();
        let db: mongodb::Database = client.database(database_name);
        let user_col: Collection<User> = db.collection::<User>(user_collection_name);
        let registration_col: Collection<RegistrationTable> = db.collection::<RegistrationTable>(reg_table_collection_name);
        let device_col: Collection<Device> = db.collection::<Device>(device_collection_name);
        let controllable_col: Collection<Controllable> = db.collection::<Controllable>(controllable_collection_name);

        Self {
            user: user_col,
            registration: registration_col,
            device: device_col,
            controllable: controllable_col
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
                    None => Err(ErrorType::UserNotFound(None))
                }
            },
            Err(err) => {
                println!("There's an error when trying to get user data. Error: {}", err.to_string());
                return Err(ErrorType::UnknownError(Some(err.to_string())));
            }
        }
    }

    pub async fn insert_registration(&self, email: &str) -> Result<RegistrationTable, ErrorType> {
        println!("[Insert Registration] Email: {}", email);

        //? Verify there's no duplicates
        let duplicated_data: Option<User> = match self.user.find_one(doc! {
            "email": email.to_string()
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
        let registration_entry = RegistrationTable::new(email.to_string());

        //? Create a query to insert the new registration data
        let query_result: Result<mongodb::results::InsertOneResult, mongodb::error::Error> = self.registration.insert_one(&registration_entry).await;


        //? Check if there's error. if not, send the ID
        match query_result {
            Ok(_) => Ok(registration_entry),
            Err(error) => Err(ErrorType::UnknownError(Some(error.to_string())))
        }
    }

    pub async fn get_confirmation_data(&self, target_id: &ObjectId, confirmation_token: &str) -> Result<RegistrationTable, ErrorType> {
        //? Create query to find the confirmation data based on it's ID and Confirmation Token
        println!("[Get Confirmation Data] Target ID: {}", target_id);
        let query_result = self.registration.find_one(doc! {
            "_id": target_id.clone(),
            "confirmation_token": confirmation_token.to_string()
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
            "confirmation_token": confirmation_token.to_string()
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

    pub async fn setup_account(&self, target_id: &ObjectId, setup_token: &str, username: &str, password: &str) -> Result<User, ErrorType> {
        //? Create query to find the confirmation data based on it's ID and Confirmation Token
        println!("[Setup Account] Target ID: {}", target_id);
        let query_result = self.registration.find_one(doc! {
            "_id": target_id.clone(),
            "setup_token": setup_token.to_string()
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
            "username": username.to_string(),
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
                return Err(ErrorType::DuplicatesFound(None))
            },
            None => ()
        };

        
        //? Create user account
        let user_data: User = User::new(username.to_string(), registration_data.email.clone(), password.to_string());
        let user_creation_result = self.user.insert_one(&user_data).await;

        match user_creation_result {
            Ok(_) => Ok(user_data),
            Err(err) => Err(ErrorType::UnknownError(Some(err.to_string())))
        }
    }

    pub async fn verify_login(&self, username: &str, password: &str) -> Result<User, ErrorType> {
        println!("Username: {}", username);

        //? Create query to get the user data based on it's username
        let query_result = self.user.find_one(doc! {
            "username": username.to_string()
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

    pub async fn initialize_device(&self, device_key: &str, device_pass: &str) -> Result<Device, ErrorType> {
        match self.device.update_one(doc! {
            "device_key": device_key,
            "device_pass": device_pass
        }, doc! {
            "$set": {
                "last_online": DateTime::now(),
                "status": 0
            }
        }).await {
            Ok(res) => {
                match res.matched_count {
                    0 => {
                        Err(ErrorType::DeviceNotFound(None))
                    },
                    _ => {
                        let device_data = match self.device.find_one(doc! {
                            "device_key": device_key,
                            "device_pass": device_pass
                        }).await {
                            Ok(res) => match res {
                                Some(data) => data,
                                None => {
                                    return Err(ErrorType::DeviceNotFound(None));
                                }
                            },
                            Err(err) => {
                                println!("There's an error when trying to get device data. Error: {}", err.to_string());
                                return Err(ErrorType::UnknownError(None));
                            }
                        };

                        Ok(device_data)
                    }
                }
            },
            Err(err) => {
                println!("There's an error when trying to update device data. Error: {}", err.to_string());
                Err(ErrorType::UnknownError(Some(err.to_string())))
            }
        }
    }

    pub async fn create_device(&self, device_name: &str, user_email: &str) -> Result<Device, ErrorType> {
        let device_data = Device::new(device_name.to_string(), user_email.to_string());
        match self.device.insert_one(&device_data).await {
            Ok(_) => Ok(device_data),
            Err(err) => {
                println!("There's an error when trying to insert device data. Error: {}", err.to_string());
                Err(ErrorType::UnknownError(None))
            }
        }
    }

    pub async fn create_controllable(&self, device_id: &str, controllable_name: &str, controllable_category: ControllableCategory, user_email: &str) -> Result<Controllable, ErrorType> {
        //? Get and Verify the controllable category
        let unexpected_controllable_data = self.controllable.find_one(doc! {
            "controllable_name": controllable_name
        }).await;

        let controllable_data = match unexpected_controllable_data {
            Ok(res) => res,
            Err(_) => {
                println!("There's an error when trying to get controllable");
                return Err(ErrorType::UnknownError(None));
            }
        };

        match controllable_data {
            Some(_) => {
                return Err(ErrorType::DuplicatesFound(None));
            },
            None => ()
        };

        //? Create the controllable_data
        let object_device_id = ObjectId::parse_str(device_id);
        let object_device_id = match object_device_id {
            Ok(res) => res,
            Err(err) => {
                println!("There's an error when trying to parse device_id");
                return Err(ErrorType::UnknownError(Some(err.to_string())));
            },
        };
        
        let controllable_data = Controllable::new(controllable_name.to_string(), controllable_category, object_device_id, user_email.to_string());
        let create_result = self.controllable.insert_one(&controllable_data).await;

        match create_result {
            Ok(_) => Ok(controllable_data),
            Err(err) => {
                println!("There's an error when trying to create controllable data");
                Err(ErrorType::UnknownError(Some(err.to_string())))
            },
        }
    }

    pub async fn get_controllable(&self, controllable_name: &str) -> Result<Controllable, ErrorType> {
        //? Get the controllable data
        let controllable_data: Result<Option<Controllable>, _> = self.controllable.find_one(doc! {
            "controllable_name": controllable_name
        }).await;
        
        // Check if there's an error
        let controllable_data: Option<Controllable> = match controllable_data {
            Ok(res) => res,
            Err(err) => {
                println!("There's an error when trying to get controllable data. Error: {}", err.to_string());
                return Err(ErrorType::UnknownError(Some(err.to_string())))
            }
        };

        // Check if there's no controllable found
        let controllable_data: Controllable = match controllable_data {
            Some(res) => res,
            None => {
                return Err(ErrorType::ControllableNotFound(None));
            }
        };

        Ok(controllable_data)
    }

    pub async fn verify_device_key_pass(&self, device_key: &str, device_pass: &str) -> Result<Device, ErrorType> {
        let device_data = match self.device.find_one(doc! {
            "device_key": device_key,
            "device_pass": device_pass
        }).await {
            Ok(res) => res,
            Err(err) => {
                println!("There's an error when trying to get device data. Error: {}", err.to_string());
                return Err(ErrorType::UnknownError(Some(err.to_string())));
            }
        };

        match device_data {
            Some(res) => Ok(res),
            None => Err(ErrorType::DeviceNotFound(None)),
        }
    }
}