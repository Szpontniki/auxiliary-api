use crate::routes::route::Route;
use api_schema::models::{ProcessImageRequest, ProcessImageResponse};
use axum::{routing::get, extract::Json};

// TODO: implement this!!!
async fn handler(Json(payload): Json<ProcessImageRequest>) -> Json<ProcessImageResponse> {
    Json(ProcessImageResponse {
        image_base64: vec![1],
        pixels: vec![],
    })
}

pub fn route() -> Route {
    Route::new(
        String::from("/process/image"),
        get(handler)
    )
}
