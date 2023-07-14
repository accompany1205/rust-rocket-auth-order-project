// @generated automatically by Diesel CLI.

diesel::table! {
    orders (id) {
        id -> Int4,
        price -> Int4,
        quantity -> Int4,
        side -> Varchar,
        user_id -> Nullable<Int4>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    orders,
    users,
);
