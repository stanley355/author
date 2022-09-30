// @generated automatically by Diesel CLI.

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
