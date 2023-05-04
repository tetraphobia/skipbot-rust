// @generated automatically by Diesel CLI.

diesel::table! {
    quotes (id) {
        id -> Integer,
        created_at -> Nullable<Timestamp>,
        quoted_by -> Text,
        quote_string -> Text,
        author -> Text,
    }
}
