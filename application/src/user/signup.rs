use domain::models::{User, NewUser};
use shared::response_models::{Response, ResponseBody};
use infrastructure::establish_connection;
use diesel::prelude::*;
use rocket::response::status::Created;
use rocket::serde::json::Json;

pub fn sign_up(user: Json<NewUser>) -> Created<String> {
    use domain::schema::users;

    let user = user.into_inner();

    match diesel::insert_into(users::table).values(&user).get_result::<User>(&mut establish_connection()) {
        Ok(user) => {
            let response = Response { body: ResponseBody::Message(format!("Successfully Created!-{}", user.email)) };
            Created::new("").tagged_body(serde_json::to_string(&response).unwrap())
        },
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        }
    }
}