// @generated automatically by Diesel CLI.

diesel::table! {
    balance_logs (id) {
        id -> Uuid,
        user_id -> Uuid,
        created_at -> Timestamp,
        prev_balance -> Float8,
        increase_amount -> Float8,
        decrease_amount -> Float8,
        final_balance -> Float8,
    }
}

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

diesel::joinable!(balance_logs -> users (user_id));
diesel::joinable!(prompts -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    balance_logs,
    prompts,
    users,
);
