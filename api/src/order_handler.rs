use shared::response_models::{Response, ResponseBody};
use application::order::{create, read, delete};
use domain::models::{Order, NewOrder};
use rocket::{get, post};
use rocket::response::status::NotFound;
use rocket::response::status::Created;
use rocket::serde::json::Json;

#[get("/")]
pub fn list_orders_handler() -> String {
    let orders: Vec<Order> = read::list_orders();
    let response = Response { body: ResponseBody::Orders(orders) };

    serde_json::to_string(&response).unwrap()
}

#[get("/order/<order_id>")]
pub fn list_order_handler(order_id: i32) -> Result<String, NotFound<String>> {
    let order = read::list_order(order_id)?;
    let response = Response { body: ResponseBody::Order(order) };

    Ok(serde_json::to_string(&response).unwrap())
}

#[post("/new_order", format = "application/json", data = "<order>")]
pub fn create_order_handler(order: Json<NewOrder>) -> Created<String> {
    create::create_order(order)
}

#[get("/delete/<order_id>")]
pub fn delete_order_handler(order_id: i32) -> Result<String, NotFound<String>> {
    let orders = delete::delete_order(order_id)?;
    let response = Response { body: ResponseBody::Orders(orders) };

    Ok(serde_json::to_string(&response).unwrap())
}