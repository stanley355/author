// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Uuid,
        fullname -> Varchar,
        email -> Varchar,
        password -> Varchar,
        phone_number -> Nullable<Varchar>,
    }
}
