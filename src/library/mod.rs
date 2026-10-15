use crate::database::models::library_entry::MediaType;
use crate::database::{self, models::library_entry::NewLibraryEntry};
use crate::database::schema::library_entries;
use crate::process::frame::Pixels;
use crate::storage::bucket::Bucket;
use crate::utils;
use std::io::Cursor;
use aws_sdk_s3::primitives::ByteStream;
use image::{DynamicImage, ImageFormat};
use diesel::RunQueryDsl;
use uuid::Uuid;

fn image_object_to_bytes(image_object: DynamicImage) -> Vec<u8> {
    let mut cursor = Cursor::new(Vec::new());
    image_object
        .write_to(&mut cursor, ImageFormat::Png)
        .expect("Failed to encode image");
    cursor.into_inner()
}

pub async fn create_image_entry(processed_pixels: Pixels, processed_image_object: DynamicImage) {
    let bytes = image_object_to_bytes(processed_image_object);
    let byte_stream = ByteStream::from(bytes);

    let file_reference_id = utils::file_reference::upload_file(Bucket::Library, "png", "image/png", byte_stream).await;
    let library_entry_id = Uuid::new_v4();

    let mut db_connection = database::connection();

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
