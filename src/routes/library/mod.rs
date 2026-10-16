use crate::database::models::file_reference::FileReference;
use crate::database::models::library_entry::LibraryEntry;
use crate::library;
use crate::routes::route::Route;
use api_schema::models::{
    GetLibraryEntries200Response as GetLibraryResponse,
    GetLibraryEntries200ResponseEntriesInner as GetLibraryResponseEntries,
    GetLibraryEntries200ResponseEntriesInnerFileReference as GetLibraryResponseEntriesFileReference,
};
use axum::{Json, routing::get};

fn type_cast_library_entries(library_entry_pairs: Vec<(LibraryEntry, FileReference)>) -> Vec<GetLibraryResponseEntries> {
    let mut new_entries: Vec<GetLibraryResponseEntries> = vec![];

    for pair in library_entry_pairs {
        let (entry, file_reference) = pair;

        let file_type_string = format!("{:?}", entry.file_type).to_lowercase();
        new_entries.push(GetLibraryResponseEntries {
            id: entry.id.to_string(),
            file_reference: Box::new(GetLibraryResponseEntriesFileReference {
                id: file_reference.id.to_string(),
                size_bytes: file_reference.size_bytes,
                mime_type: file_reference.mime_type,
                path: file_reference.path,
                bucket: file_reference.bucket,
                created_at: file_reference.created_at.to_rfc3339(),
                updated_at: file_reference.updated_at.to_rfc3339(),
            }),
            processed_output: serde_json::from_value(entry.processed_output).unwrap(),
            file_type: file_type_string,
            created_at: entry.created_at.to_rfc3339(),
            updated_at: entry.updated_at.to_rfc3339(),
        });
    }

    new_entries
}

async fn handler() -> Json<GetLibraryResponse> {
    let library_entries = library::get_recent_entries().await;
    
    Json(GetLibraryResponse {
        entries: type_cast_library_entries(library_entries),
    })
}

fn route() -> Route {
    Route::new(
        String::from("/library"),
        get(handler)
    )
}

pub fn routes() -> Vec<Route> {
    vec![route()]
}
