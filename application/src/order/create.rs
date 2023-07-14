use domain::models::{Order, NewOrder};
use shared::response_models::{Response, ResponseBody};
use infrastructure::establish_connection;
use diesel::prelude::*;
use rocket::response::status::Created;
use rocket::serde::json::Json;

pub fn create_order(order: Json<NewOrder>) -> Created<String> {
    use domain::schema::orders;

    let order = order.into_inner();

    match diesel::insert_into(orders::table).values(&order).get_result::<Order>(&mut establish_connection()) {
        Ok(order) => {
            let response = Response { body: ResponseBody::Order(order) };
            Created::new("").tagged_body(serde_json::to_string(&response).unwrap())
        },
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        }
    }
}