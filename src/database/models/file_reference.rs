use crate::database::schema::file_references;
use diesel::prelude::*;

use chrono::{DateTime, Utc};
use uuid::Uuid;


#[derive(Queryable, Selectable)]
#[diesel(table_name = file_references)]
pub struct FileReference {
    pub id: Uuid,
    pub size_bytes: i32,
    pub mime_type: String,
    pub bucket: String,
    pub path: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Insertable)]
#[diesel(table_name = file_references)]
pub struct NewFileReference {
    pub size_bytes: i32,
    pub mime_type: String,
    pub bucket: String,
    pub path: String,
}
