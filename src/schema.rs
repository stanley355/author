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
        total_token -> Int4,
        total_cost -> Float8,
        instruction -> Varchar,
    }
}

diesel::table! {
    subscriptions (id) {
        id -> Uuid,
        user_id -> Uuid,
        topup_id -> Uuid,
        created_at -> Timestamp,
        start_at -> Timestamp,
        end_at -> Timestamp,
        duration_type -> Varchar,
        paid -> Bool,
        is_paylater -> Nullable<Bool>,
    }
}

diesel::table! {
    topups (id) {
        id -> Uuid,
        user_id -> Uuid,
        created_at -> Timestamp,
        topup_amount -> Float8,
        paid -> Bool,
        topup_type -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        fullname -> Varchar,
        email -> Varchar,
        password -> Varchar,
        phone_number -> Nullable<Varchar>,
        balance -> Float8,
    }
}

diesel::joinable!(prompts -> users (user_id));
diesel::joinable!(subscriptions -> topups (topup_id));
diesel::joinable!(subscriptions -> users (user_id));
diesel::joinable!(topups -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    prompts,
    subscriptions,
    topups,
    users,
);
