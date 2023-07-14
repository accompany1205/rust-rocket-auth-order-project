diesel::table! {
    orders (id) {
        id -> Int4,
        price -> Int4,
        quantity -> Int4,
        side -> Varchar,
        user_id -> Int4,
    }
}
diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar
    }
}