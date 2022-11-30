// @generated automatically by Diesel CLI.

diesel::table! {
    links (id) {
        id -> Integer,
        resolved_title -> Text,
        resolved_url -> Text,
        resolved_status -> Integer,
        added_on -> Text,
        item_id -> Text,
    }
}

diesel::table! {
    settings (id) {
        id -> Integer,
        key -> Text,
        value -> Nullable<Text>,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        username -> Text,
        email -> Text,
        created_on -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(links, settings, users,);
