// @generated automatically by Diesel CLI.

diesel::table! {
    books (id) {
        id -> Integer,
        name -> Text,
        category -> Text,
        created_at -> Timestamp,
    }
}
