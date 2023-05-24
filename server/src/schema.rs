// @generated automatically by Diesel CLI.

diesel::table! {
    user_chat (id) {
        id -> Int4,
        user_id -> Int4,
        chat_id -> Varchar,
        date_joined -> Timestamp,
        is_admin -> Bool,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        first_name -> Varchar,
        last_name -> Varchar,
        email -> Varchar,
        password -> Varchar,
        phone -> Varchar,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        active -> Nullable<Bool>,
        superuser -> Nullable<Bool>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    user_chat,
    users,
);
