// @generated automatically by Diesel CLI.

diesel::table! {
    checkbots (id) {
        id -> Int4,
        user_id -> Uuid,
        created_at -> Timestamp,
        source_text_token -> Int4,
        checkbot_text_token -> Int4,
        source_text -> Varchar,
        checkbot_text -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        fullname -> Varchar,
        email -> Varchar,
        password -> Varchar,
        phone_number -> Nullable<Varchar>,
    }
}

diesel::joinable!(checkbots -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    checkbots,
    users,
);
