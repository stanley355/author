// @generated automatically by Diesel CLI.

diesel::table! {
    prompts (id) {
        id -> Int4,
        user_id -> Uuid,
        created_at -> Timestamp,
        prompt_token -> Int4,
        completion_token -> Int4,
        prompt_text -> Varchar,
        completion_text -> Varchar,
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

diesel::joinable!(prompts -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    prompts,
    users,
);
