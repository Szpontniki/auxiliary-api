use crate::routes::route::Route;
use crate::process::frame::FrameProcessor;
use api_schema::models::{ProcessImage200ResponsePixelsInner, ProcessImageRequest, ProcessImageResponse};
use axum::{routing::post, extract::Json};

async fn handler(Json(payload): Json<ProcessImageRequest>) -> Json<ProcessImageResponse> {
    let mut processor = FrameProcessor::from_base_64(payload.image_base64);

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
        image_base64: processor.as_base_64(),
        pixels: expected_pixels,
    })
}

pub fn route() -> Route {
    Route::new(
        String::from("/process/image"),
        post(handler)
    )
}
