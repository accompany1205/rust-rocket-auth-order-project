#[macro_use]
extern crate rocket;
use rocket::http::{Header, Method, Status};
use rocket::{ Request, Response };
use rocket::fairing::{ Fairing, Info, Kind };

use api::auth_handler;
use api::order_handler;

pub struct Cors;

#[rocket::async_trait]
impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "Cross-Origin-Resource-Sharing Fairing",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        if _request.uri().path().starts_with("/api/orders") || _request.uri().path().starts_with("/api/user") {
            response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
            response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, OPTIONS"));
            response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
            response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
            // Set the HTTP status to 200 OK for OPTIONS requests
            if response.status() == Status::NotFound && _request.method() == Method::Options {
                response.set_status(Status::Ok);
            }
        }
    }
}

#[launch]
fn rocket() -> _ {
    rocket
        ::build()
        .attach(Cors)
        .mount(
            "/api/orders",
            routes![
                order_handler::list_orders_handler,
                order_handler::list_order_handler,
                order_handler::create_order_handler,
                order_handler::delete_order_handler
            ]
        )
        .mount(
            "/api/user",
            routes![
                auth_handler::login_handler,
                auth_handler::signup_handler,
            ]
        )
}
