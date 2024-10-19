// @generated automatically by Diesel CLI.

diesel::table! {
    checkbots (id) {
        id -> Int4,
        user_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        instruction -> Varchar,
        model -> Varchar,
        system_content -> Text,
        user_content -> Text,
        completion_content -> Text,
        prompt_tokens -> Int4,
        completion_tokens -> Int4,
        total_tokens -> Int4,
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
        instruction -> Varchar,
        prompt_type -> Nullable<Varchar>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    speech_to_text (id) {
        id -> Int4,
        user_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        model -> Varchar,
        file_name -> Varchar,
        file_url -> Varchar,
        language -> Varchar,
        transcription -> Varchar,
        timestamp_granularity -> Nullable<Varchar>,
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
    text_to_speech (id) {
        id -> Int4,
        user_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        model -> Varchar,
        input -> Varchar,
        voice -> Varchar,
        speed -> Int4,
        response_format -> Varchar,
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
    translation (id) {
        id -> Int4,
        user_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        content_language -> Varchar,
        target_language -> Varchar,
        model -> Varchar,
        system_content -> Text,
        user_content -> Text,
        completion_content -> Text,
        prompt_tokens -> Int4,
        completion_tokens -> Int4,
        total_tokens -> Int4,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        fullname -> Varchar,
        email -> Varchar,
        password -> Varchar,
        phone_number -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(checkbots -> users (user_id));
diesel::joinable!(prompts -> users (user_id));
diesel::joinable!(speech_to_text -> users (user_id));
diesel::joinable!(students -> users (user_id));
diesel::joinable!(subscriptions -> users (user_id));
diesel::joinable!(text_to_speech -> users (user_id));
diesel::joinable!(topups -> users (user_id));
diesel::joinable!(translation -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    checkbots,
    prompts,
    speech_to_text,
    students,
    subscriptions,
    text_to_speech,
    topups,
    translation,
    users,
);
