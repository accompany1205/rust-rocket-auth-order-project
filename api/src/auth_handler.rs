use shared::response_models::{ Response, ResponseBody };
use application::user::{ login, signup };
use domain::models::{ NewUser, LoginUser };
use rocket::{ post };
use rocket::response::status::NotFound;
use rocket::response::status::Created;
use rocket::serde::json::Json;

#[post("/signup", format = "application/json", data = "<user>")]
pub fn signup_handler(user: Json<NewUser>) -> Created<String> {
    signup::sign_up(user)
}

#[post("/login", format = "application/json", data = "<user_info>")]
pub fn login_handler(user_info: Json<LoginUser>) -> Result<String, NotFound<String>> {
    let result = login::log_in(user_info)?;
    let response = Response { body: ResponseBody::LoginResponse(result) };
    Ok(serde_json::to_string(&response).unwrap())
}
