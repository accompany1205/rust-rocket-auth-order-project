use shared::response_models::{Response, ResponseBody};
use infrastructure::establish_connection;
use diesel::prelude::*;
use rocket::response::status::NotFound;
use domain::models::Order;

pub fn delete_order(order_id: i32) -> Result<Vec<Order>, NotFound<String>> {
    use domain::schema::orders::dsl::*;
    use domain::schema::orders;

    let response: Response;

    let num_deleted = match diesel::delete(orders.filter(id.eq(order_id))).execute(&mut establish_connection()) {
        Ok(count) => count,
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let response = Response { body: ResponseBody::Message(format!("Error deleting order with id {} - {}", order_id, err))};
                return Err(NotFound(serde_json::to_string(&response).unwrap()));
            },
            _ => {
                panic!("Database error - {}", err);
            }        
        }
    };

    if num_deleted > 0 {
        match orders::table.select(orders::all_columns).load::<Order>(&mut establish_connection()) {
            Ok(mut orders_) => {
                orders_.sort();
                Ok(orders_)
            },
            Err(err) => match err {
                _ => {
                    panic!("Database error - {}", err);
                }
            }
        }
    } else {
        response = Response { body: ResponseBody::Message(format!("Error - no order with id {}", order_id))};
        Err(NotFound(serde_json::to_string(&response).unwrap()))
    } 
}