use crate::schema::users;
use crate::schema::orders;

use diesel::prelude::*;
use rocket::serde::{ Deserialize, Serialize };
use std::cmp::{ Ord, Eq, PartialOrd, PartialEq };

// Queryable will generate the code needed to load the struct from an SQL statement
#[derive(Queryable, Serialize, Ord, Eq, PartialEq, PartialOrd)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String
}

#[derive(Insertable, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password: String
}

#[derive(Deserialize)]
pub struct LoginUser {
    pub email: String,
    pub password: String
}

#[derive(Queryable, Serialize, Ord, Eq, PartialEq, PartialOrd)]
pub struct LoginResponse {
    pub token: String,
    pub user_id: i32
}

#[derive(Queryable, Serialize, Ord, Eq, PartialEq, PartialOrd)]
pub struct Order {
    pub id: i32,
    pub price: i32,
    pub quantity: i32,
    pub side: String,
    pub user_id: i32,
}

#[derive(Insertable, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = orders)]
pub struct NewOrder {
    pub price: i32,
    pub quantity: i32,
    pub side: String,
    pub user_id: i32,
}
