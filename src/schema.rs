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
        prompt_type -> Nullable<Varchar>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    students (id) {
        id -> Uuid,
        user_id -> Uuid,
        student_id -> Varchar,
        student_email -> Nullable<Varchar>,
        student_card_img_url -> Nullable<Varchar>,
        institution_level -> Varchar,
        institution_name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        free_discount_end_at -> Timestamp,
        half_discount_end_at -> Timestamp,
    }
}

diesel::table! {
    subscriptions (id) {
        id -> Uuid,
        user_id -> Uuid,
        created_at -> Timestamp,
        start_at -> Timestamp,
        end_at -> Timestamp,
        duration_type -> Varchar,
        paid -> Bool,
        updated_at -> Nullable<Timestamp>,
        price -> Float8,
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
        updated_at -> Nullable<Timestamp>,
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
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(prompts -> users (user_id));
diesel::joinable!(students -> users (user_id));
diesel::joinable!(subscriptions -> users (user_id));
diesel::joinable!(topups -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    prompts,
    students,
    subscriptions,
    topups,
    users,
);
