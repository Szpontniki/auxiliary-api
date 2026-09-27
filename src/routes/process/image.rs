use crate::routes::route::Route;
use crate::process::frame::FrameProcessor;
use api_schema::models::{ProcessImage200ResponsePixelsInner, ProcessImageRequest, ProcessImageResponse};
use axum::{routing::post, extract::Json};
use base64::{engine::general_purpose::STANDARD, Engine as _};

async fn handler(Json(payload): Json<ProcessImageRequest>) -> Json<ProcessImageResponse> {
    let base64 = STANDARD.encode(&payload.image_base64);
    let mut processor = FrameProcessor::from_base_64(base64);

    processor.resize(payload.width as u32, payload.height as u32);

    // TODO: we shouldn't have to do this. This is due to an issue with our
    // current api-schema or the specific generator used in this repo.
    let expected_pixels: Vec<ProcessImage200ResponsePixelsInner> = processor.as_pixels()
        .into_iter()
        .map(|p| ProcessImage200ResponsePixelsInner {
            // Ridiculous.
            x: p.x,
            y: p.y,
            color: p.color,
        })
        .collect();

    Json(ProcessImageResponse {
        image_base64: processor.as_bytes(),
        pixels: expected_pixels,
    })
}

pub fn route() -> Route {
    Route::new(
        String::from("/process/image"),
        post(handler)
    )
}
