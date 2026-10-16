// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "media_type"))]
    pub struct MediaType;
}

diesel::table! {
    file_references (id) {
        id -> Uuid,
        size_bytes -> Int4,
        mime_type -> Text,
        bucket -> Text,
        path -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        public_url -> Nullable<Text>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::MediaType;

    library_entries (id) {
        id -> Uuid,
        file_reference_id -> Uuid,
        file_type -> MediaType,
        processed_output -> Jsonb,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::joinable!(library_entries -> file_references (file_reference_id));

diesel::allow_tables_to_appear_in_same_query!(
    file_references,library_entries,);
