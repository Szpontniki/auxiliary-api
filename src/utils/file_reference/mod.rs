use crate::database::models::file_reference::NewFileReference;
use crate::database::schema::file_references;
use crate::database;
use crate::storage::bucket::Bucket;
use crate::storage::storage;
use aws_sdk_s3::primitives::ByteStream;
use diesel::RunQueryDsl;
use uuid::Uuid;

// Returns file reference UUID.
pub async fn upload_file(bucket: Bucket, extension: &str, content_type: &str, content: ByteStream) -> Uuid {
    let mut db_connection = database::connection();
    let storage = storage().await;

    let file_reference_id = Uuid::new_v4();
    let file_path = format!("{}.{}", file_reference_id.to_string(), extension);

    let bytes = content.collect().await.unwrap().into_bytes();
    let size_bytes = i32::try_from(bytes.len()).unwrap();

    let new_file_reference = NewFileReference {
        id: file_reference_id,
        path: file_path.clone(),
        bucket: bucket.as_str(),
        mime_type: String::from(content_type),
        size_bytes,
    };

    diesel::insert_into(file_references::table)
        .values(&new_file_reference)
        .execute(&mut db_connection)
        .unwrap();

    storage.upload_file(bucket, &file_path, content_type, ByteStream::from(bytes)).await;

    file_reference_id
}
