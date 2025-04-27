use crate::{db::Database, middlewares::security::ApiKey, types::{api::{ResponseBody, ResponseBodyType}, db_model::{RegistrationTable, User}, error::ErrorType}, utils::{self, create_user_token, verify_user_token_from_cookie}};
use lettre::{message::Mailbox, transport::smtp::authentication::Credentials, Message, SmtpTransport, Transport};
use mongodb::bson::oid::ObjectId;
use rocket::{http::{self, Cookie, CookieJar}, response::status, serde::json::Json, State};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UserRegistrationBody {
    pub email: String
}

#[derive(Serialize, Deserialize)]
pub struct ConfirmRegistrationBody {
    pub id: ObjectId,
    pub token: String
}

#[derive(Serialize, Deserialize)]
pub struct SetupRegistrationBody {
    pub id: ObjectId,
    pub token: String,
    pub username: String,
    pub password: String
}

#[derive(Serialize, Deserialize)]
pub struct LoginBody {
    pub username: String,
    pub password: String
}

#[derive(Serialize, Deserialize)]
pub struct CreateDeviceBody {
    pub device_name: String
}


#[post("/user/registration", data = "<body_data>")]
pub async fn user_registration(_api_key: ApiKey, db: &State<Database>, body_data: Json<UserRegistrationBody>) -> status::Custom<Json<ResponseBody>> {
    //? Get the required data
    let user_email = &body_data.email;
    println!("Incoming Email: {}", user_email);
    
    //? Check if the email is valid or not.
    if !utils::is_valid_email(user_email.as_str()) {
        return status::Custom(http::Status::BadRequest, Json(ResponseBody { message: format!("Email is not valid!"), success: false, data: None }));
    }


    //? Store the confirmation token to database
    let registration_data: RegistrationTable = match db.insert_registration(user_email).await {
        Ok(result) => result,
        Err(error) => {
            match error {
                ErrorType::DuplicatesFound(_) => {
                    return status::Custom(http::Status::Conflict, Json(ResponseBody { message: format!("There's duplicate found!"), success: false, data: None }));
                },
                _ => ()
            }
            return status::Custom(http::Status::InternalServerError, Json(ResponseBody { message: format!("Sorry, there's an unexpected error"), success: false, data: None }));
        }
    };


    //? Send the token to targetted email
    let email_user: String = std::env::var("EMAIL_USER").expect("Please, define 'EMAIL_USER' in your .env");
    let email_pass: String = std::env::var("EMAIL_PASS").expect("Please, define 'EMAIL_PASS' in your .env");
    
    let email_from: Mailbox = match format!("ROVI Project <{}>", email_user).parse::<Mailbox>() {
        Ok(res) => res,
        Err(err) => {
            println!("There's an error when trying to build email_from data. Error: {}", err.to_string());
            return status::Custom(http::Status::InternalServerError, Json(ResponseBody { message: format!("There's an error when sending email!"), success: false, data: None }));
        }
    };

    let email_to: Mailbox = match user_email.parse::<Mailbox>() {
        Ok(res) => res,
        Err(err) => {
            println!("There's an error when trying to build email_to data. Error: {}", err.to_string());
            return status::Custom(http::Status::InternalServerError, Json(ResponseBody { message: format!("There's an error when sending email!"), success: false, data: None }));
        }
    };
    
    let email = match Message::builder()
        .from(email_from)
        .to(email_to)
        .subject("Account Confirmation")
        .body(format!("Hi there, Thank you for signing up to ROVI Project! Please use token below to proceed:<br /><b>TOKEN:[{}]</b>", registration_data.confirmation_token)) {
            Ok(res) => res,
            Err(err) => {
                println!("There's an error when trying to build message data. Error: {}", err);
                return status::Custom(http::Status::InternalServerError, Json(ResponseBody { message: format!("There's an error when sending email!"), success: false, data: None }));
            }
        };

    let creds = Credentials::new(
        email_user.to_string(),
        email_pass.to_string(),
    );
    
    let mailer = match SmtpTransport::relay("smtp.gmail.com") {
        Ok(res) => res,
        Err(err) => {
            println!("There's an error when trying to build SMTP Transport. Error: {}", err);
            return status::Custom(http::Status::InternalServerError, Json(ResponseBody { message: format!("There's an error when sending email!"), success: false, data: None }));
        }
    }
        .credentials(creds)
        .build();

    match mailer.send(&email) {
        Ok(_) => (),
        Err(error) => {
            println!("There's an error when trying to send data: {error}");
            return status::Custom(http::Status::InternalServerError, Json(ResponseBody { message: format!("There's an error when sending email!"), success: false, data: None }))
        },
    };

    //? Success
    status::Custom(http::Status::Ok, Json(ResponseBody { message: format!("Successfully sent email confirmation to {}!", user_email.as_str()), success: true, data: Some(ResponseBodyType::UserRegistration { id: registration_data.id.to_string() }) }))
}


#[post("/user/confirm_registration", data = "<body_data>")]
pub async fn confirm_registration(_api_key: ApiKey, db: &State<Database>, body_data: Json<ConfirmRegistrationBody>) -> status::Custom<Json<ResponseBody>> {
    //? Get the required data
    let target_id = &body_data.id;
    let confirmation_token = &body_data.token;
    
    //? Get the confirmation data
    let registration_data: RegistrationTable = match db.get_confirmation_data(target_id, confirmation_token).await {
        Ok(res) => res,
        Err(err) => {
            match err {
                ErrorType::UnknownError(message) => {
                    println!("There's an error when trying to verify registration. Error: {}", message.unwrap_or(format!("")));
                },
                ErrorType::Unauthorized(_) => {
                    return status::Custom(http::Status::Unauthorized, Json(ResponseBody { message: format!("Wrong token."), success: false, data: None }))
                },
                _ => ()
            };
            return status::Custom(http::Status::InternalServerError, Json(ResponseBody { message: format!("There's an unexpected error."), success: false, data: None }))
        }
    };

    //? If verified, send the setup token
    status::Custom(http::Status::Ok, Json(ResponseBody { message: format!("Successfully verify the registration token!"), success: true, data: Some(ResponseBodyType::UserVerify { token: registration_data.setup_token, id: registration_data.id.to_string() }) }))
}


#[post("/user/setup_registration", data = "<body_data>")]
pub async fn setup_registration(_api_key: ApiKey, db: &State<Database>, body_data: Json<SetupRegistrationBody>, cookies: &CookieJar<'_>) -> status::Custom<Json<ResponseBody>> {
    //? Get the required data
    let target_id = &body_data.id;
    let setup_token = &body_data.token;
    let username = &body_data.username;
    let password = &body_data.password;

    //? Setup account
    let result = db.setup_account(target_id, setup_token, username, password).await;
    match result {
        Ok(user_data) => {
            cookies.add(Cookie::new("user_token", create_user_token(user_data.email.as_str())));
            status::Custom(http::Status::Ok, Json(ResponseBody { message: format!("Successfully register!"), success: true, data: Some(ResponseBodyType::UserSetup { user_data }) }))
        },
        Err(err) => {
            match err {
                ErrorType::UnknownError(message) => {
                    println!("There's an error when trying to setup registration. Error: {}", (message.unwrap_or(format!(""))));
                },
                ErrorType::Unauthorized(_) => {
                    return status::Custom(http::Status::Unauthorized, Json(ResponseBody { message: format!("Wrong token."), success: false, data: None }))
                },
                ErrorType::DuplicatesFound(_) => {
                    return status::Custom(http::Status::Conflict, Json(ResponseBody { message: format!("Duplicates found."), success: false, data: None }))
                },
                _ => ()
            };
            status::Custom(http::Status::InternalServerError, Json(ResponseBody { message: format!("There's an unexpected error."), success: false, data: None }))
        }  
    }
}


#[post("/user/login", data = "<body_data>")]
pub async fn user_login(_api_key: ApiKey, db: &State<Database>, body_data: Json<LoginBody>, cookies: &CookieJar<'_>) -> status::Custom<Json<ResponseBody>> {
    //? Get the required data
    let username = &body_data.username;
    let password = &body_data.password;

    //? Verify login
    let login_result = db.verify_login(username, password).await;

    match login_result {
        Ok(res) => {
            cookies.add(Cookie::new("user_token", create_user_token(res.email.as_str())));
            return status::Custom(http::Status::Ok, Json(ResponseBody { message: format!("Successfully login."), success: true, data: Some(ResponseBodyType::UserLogin { user_data: res }) }));
        },
        Err(err) => {
            match err {
                ErrorType::Unauthorized(_) => {
                    return status::Custom(http::Status::InternalServerError, Json(ResponseBody { message: format!("There's an unexpected error."), success: false, data: None }))
                },
                _ => ()
            };

            status::Custom(http::Status::InternalServerError, Json(ResponseBody { message: format!("There's an unexpected error."), success: false, data: None }))
        }
    }
}


#[get("/user/get")]
pub async fn user_get(_api_key: ApiKey, db: &State<Database>, cookies: &CookieJar<'_>) -> status::Custom<Json<ResponseBody>> {
    //? Fetch user's email from user_token cookie!
    let user_email: String = match verify_user_token_from_cookie(cookies) {
        Ok(res) => res.clone(),
        Err(_) => {
            return status::Custom(http::Status::Unauthorized, Json(ResponseBody { message: format!("Unauthorized."), success: false, data: None }));
        }
    };

    
    //? Get user data based on the user email that we've just got! :D
    let user_data: User = match db.get_user(user_email.as_str()).await {
        Ok(res) => res,
        Err(err) => {
            match err {
                ErrorType::UserNotFound(_) => {
                    return status::Custom(http::Status::NotFound, Json(ResponseBody { message: format!("User not found."), success: false, data: None }))
                },
                ErrorType::UnknownError(_) => {
                    return status::Custom(http::Status::InternalServerError, Json(ResponseBody { message: format!("There's an unexpected error."), success: false, data: None }))
                },
                _ => {
                    return status::Custom(http::Status::BadRequest, Json(ResponseBody { message: format!("There's a request error."), success: false, data: None }))
                }
            }
        }
    };


    //? Return the user data that we've just got! :)
    status::Custom(http::Status::Ok, Json(ResponseBody { message: format!("Successfully get user data"), success: true, data: Some(ResponseBodyType::UserGet { user_data: user_data }) }))
}

#[post("/user/create_device", data = "<body_data>")]
pub async fn create_device(body_data: Json<CreateDeviceBody>, _api_key: ApiKey, db: &State<Database>, cookies: &CookieJar<'_>) -> status::Custom<Json<ResponseBody>> {
    match verify_user_token_from_cookie(cookies) {
        Ok(_) => (),
        Err(_) => {
            return status::Custom(http::Status::Unauthorized, Json(ResponseBody { message: format!("Unauthorized."), success: false, data: None }));
        }
    };
    
    let device_name = &body_data.device_name;

    match db.create_device(device_name).await {
        Ok(res) => status::Custom(http::Status::Ok, Json(ResponseBody { message: format!("Successfully create device!"), success: true, data: Some(ResponseBodyType::CreateDevice { device_data: res }) })),
        Err(_) => status::Custom(http::Status::InternalServerError, Json(ResponseBody { message: format!("There's an unexpected error."), success: false, data: None }))
    }
}