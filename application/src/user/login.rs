use domain::models::{ User, LoginUser, LoginResponse };
use shared::response_models::{ Response, ResponseBody };
use infrastructure::establish_connection;
use diesel::prelude::*;
use rocket::serde::json::Json;
use rocket::response::status::NotFound;
use jsonwebtoken::{ encode, EncodingKey, Header };
use std::env;

pub fn log_in(user_info: Json<LoginUser>) -> Result<LoginResponse, NotFound<String>> {
    use domain::schema::users;
    let email = user_info.email.clone();
    let password = user_info.password.clone();
    let secret_key = "rustrocketbackend";
    // let secret_key = env::var("SECRET_KEY").expect("SECRET_KEY must be set.");
    match
        users::table
            .filter(users::email.eq(email).and(users::password.eq(password)))
            .first::<User>(&mut establish_connection())
    {
        Ok(user) => {
            match
                encode(
                    &Header::default(),
                    &user.id,
                    &EncodingKey::from_secret(secret_key.as_bytes())
                )
            {
                Ok(token) => {
                    Ok(LoginResponse {
                        token: token,
                        user_id: user.id,
                    })
                }
                Err(err) => {
                    let response = Response {
                        body: ResponseBody::Message(format!("Error encoding token: {}", err)),
                    };
                    let error_response = serde_json
                        ::to_string(&response)
                        .map_err(|err|
                            NotFound(format!("Failed to serialize error response: {}", err))
                        )?;
                    return Err(NotFound(error_response));
                }
            }
        },
        Err(err) =>
            match err {
                diesel::result::Error::NotFound => {
                    let response = Response {
                        body: ResponseBody::Message(
                            format!("Error selecting order with id {} - {}", user_info.email, err)
                        ),
                    };
                    return Err(NotFound(serde_json::to_string(&response).unwrap()));
                },
                _ => {
                    let response = Response {
                        body: ResponseBody::Message(
                            format!("Error selecting order with id {} - {}", user_info.email, err)
                        ),
                    };
                    return Err(NotFound(serde_json::to_string(&response).unwrap()));
                }
            }
    }
}
