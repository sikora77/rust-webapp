// @generated automatically by Diesel CLI.

diesel::table! {
    blogposts (id) {
        id -> Int4,
        body -> Text,
        publish_date -> Timestamp,
        #[max_length = 50]
        username -> Varchar,
        image_oid -> Nullable<Oid>,
        avatar_oid -> Nullable<Oid>,
    }
}
