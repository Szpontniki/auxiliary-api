use crate::database::schema::library_entries;
use diesel::prelude::*;

use diesel_derive_enum::DbEnum;
use chrono::{DateTime, Utc};
use serde_json::Value;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, DbEnum)]
#[db_enum(existing_type_path = "crate::database::schema::sql_types::MediaType")]
pub enum MediaType {
    Image,
    Video,
}

// TODO: write a macro to avoid having to add the ID
// and timestamp columns to every model manually.
#[derive(Queryable, Selectable)]
#[diesel(table_name = library_entries)]
pub struct LibraryEntry {
    pub id: Uuid,
    pub file_reference_id: Uuid,
    pub file_type: MediaType,
    pub processed_output: Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Insertable)]
#[diesel(table_name = library_entries)]
pub struct NewLibraryEntry {
    pub file_reference_id: Uuid,
    pub file_type: MediaType,
    pub processed_output: Value,
}
