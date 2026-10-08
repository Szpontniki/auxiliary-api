use crate::database;
use crate::database::schema::{file_references, library_entries};
use crate::database::models::file_reference::NewFileReference;
use crate::database::models::library_entry::{MediaType, NewLibraryEntry};
use crate::process::frame::{Pixels, utils::get_image_bytes};
use image::DynamicImage;
use diesel::RunQueryDsl;
use uuid::Uuid;

pub fn create_image_entry(processed_pixels: Pixels, processed_image_object: DynamicImage) {
    let mut db_connection = database::connection();
    let image_bytes = get_image_bytes(&processed_image_object).unwrap();

    let file_reference_id = Uuid::new_v4();

    let new_file_reference = NewFileReference {
        id: file_reference_id,
        path: format!("{}.png", file_reference_id.to_string()),
        // TODO: replace with an enum of some sort.
        bucket: String::from("bucket"),
        mime_type: String::from("image/png"),
        size_bytes: i32::try_from(image_bytes).unwrap(),
    };

    diesel::insert_into(file_references::table)
        .values(&new_file_reference)
        .execute(&mut db_connection)
        .unwrap();

    let library_entry_id = Uuid::new_v4();

    let new_library_entry = NewLibraryEntry {
        file_type: MediaType::Image,
        file_reference_id,
        processed_output: processed_pixels
            .into_iter()
            .map(|obj| serde_json::to_value(obj).unwrap())
            .collect(),
        id: library_entry_id,
    };

    diesel::insert_into(library_entries::table)
        .values(&new_library_entry)
        .execute(&mut db_connection)
        .unwrap();
}
