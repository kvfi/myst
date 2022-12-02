// @generated automatically by Diesel CLI.

diesel::table! {
    links (id) {
        id -> Int4,
        resolved_title -> Text,
        resolved_url -> Text,
        resolved_status -> Int4,
        added_on -> Text,
        item_id -> Text,
    }
}

diesel::table! {
    settings (id) {
        id -> Int4,
        key -> Varchar,
        value -> Nullable<Text>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        email -> Varchar,
        created_on -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(links, settings, users,);
