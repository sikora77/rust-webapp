// @generated automatically by Diesel CLI.

diesel::table! {
    blogposts (id) {
        id -> Int4,
        body -> Text,
        publish_date -> Timestamp,
        #[max_length = 50]
        username -> Varchar,
        #[max_length = 255]
        image_path -> Nullable<Varchar>,
        #[max_length = 255]
        avatar_path -> Nullable<Varchar>,
    }
}
