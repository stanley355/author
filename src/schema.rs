// @generated automatically by Diesel CLI.

diesel::table! {
    documents (id) {
        id -> Uuid,
        user_id -> Uuid,
        created_at -> Timestamp,
        name -> Varchar,
        doc_type -> Varchar,
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
    referral (id) {
        id -> Int4,
        created_at -> Timestamp,
        user_id -> Uuid,
        friend_id -> Uuid,
    }
}

diesel::table! {
    topups (id) {
        id -> Uuid,
        user_id -> Uuid,
        created_at -> Timestamp,
        topup_amount -> Float8,
        paid -> Bool,
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

diesel::joinable!(documents -> users (user_id));
diesel::joinable!(prompts -> users (user_id));
diesel::joinable!(topups -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    documents,
    prompts,
    referral,
    topups,
    users,
);
