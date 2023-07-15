// @generated automatically by Diesel CLI.

diesel::table! {
    client (id) {
        id -> Uuid,
        first_name -> Nullable<Varchar>,
        last_name -> Nullable<Varchar>,
        email -> Varchar,
        hash_password -> Varchar,
    }
}
