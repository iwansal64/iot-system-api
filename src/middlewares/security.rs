use std::env;

use rocket::request::{FromRequest, Outcome};
use rocket::{Request, http::Status};

pub struct ApiKey;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        if let Some(key) = request.headers().get_one("authorization") {
            if key == env::var("API_KEY").expect("Please, define `API_KEY` in .env file") {
                return Outcome::Success(ApiKey);
            }
            else {
                return Outcome::Error((Status::Unauthorized, ()));
            }
        } else {
            Outcome::Error((Status::Unauthorized, ()))
        }
    }
}
