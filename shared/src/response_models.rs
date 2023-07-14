use domain::models::{Order, LoginResponse};
use rocket::serde::Serialize;

#[derive(Serialize)]
pub enum ResponseBody {
    Message(String),
    Order(Order),
    Orders(Vec<Order>),
    LoginResponse(LoginResponse)
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Response {
    pub body: ResponseBody,
}