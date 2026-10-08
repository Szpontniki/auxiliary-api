use crate::library;
use crate::routes::route::Route;
use crate::process::frame::{FrameProcessor, Pixels};
use api_schema::models::{ProcessImage200ResponsePixelsInner, ProcessImageRequest, ProcessImageResponse};
use axum::{routing::post, extract::Json};

// TODO: we shouldn't have to do this. This is due to an issue with our current api-schema or the
// specific generator used in this repo. The types are matching 1-to-1. In the API schema the same
// type is actually used in both places where the `Pixel` struct is used. Due to the Rust generator
// separating them we have to type cast them like here.
fn type_cast_pixels(pixels: Pixels) -> Vec<ProcessImage200ResponsePixelsInner> {
    pixels
        .into_iter()
        .map(|p| ProcessImage200ResponsePixelsInner {
            // Ridiculous.
            x: p.x,
            y: p.y,
            color: p.color,
        })
        .collect()
}

async fn handler(Json(payload): Json<ProcessImageRequest>) -> Json<ProcessImageResponse> {
    let mut processor = FrameProcessor::from_base_64(payload.image_base64);

    processor.resize(payload.width as u32, payload.height as u32);
    let expected_pixels = processor.as_pixels();
    let expected_image_obj = processor.as_image_obj();
    let expected_base64 = processor.as_base_64();

    library::create_image_entry(expected_pixels.clone(), expected_image_obj);
    
    Json(ProcessImageResponse {
        image_base64: expected_base64,
        pixels: type_cast_pixels(expected_pixels),
    })
}

pub fn route() -> Route {
    Route::new(
        String::from("/process/image"),
        post(handler)
    )
}
