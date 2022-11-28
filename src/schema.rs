// @generated automatically by Diesel CLI.

diesel::table! {
    subscriptions (id) {
        id -> Int4,
        user_id -> Uuid,
        channels_id -> Int4,
        channels_slug -> Varchar,
        created_at -> Timestamp,
        expired_at -> Nullable<Timestamp>,
        duration -> Int4,
        channels_name -> Varchar,
        status -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        fullname -> Varchar,
        email -> Varchar,
        password -> Nullable<Varchar>,
        phone_number -> Nullable<Varchar>,
        has_channel -> Bool,
    }
}

diesel::joinable!(subscriptions -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    subscriptions,
    users,
);
