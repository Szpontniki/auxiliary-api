pub mod utils;

use api_schema::models::Pixel;
use std::io::Cursor;
use base64::{engine::general_purpose::STANDARD, Engine as _};
use image::{DynamicImage, ImageFormat, imageops::FilterType};

pub type Pixels = Vec<Pixel>;

pub struct FrameProcessor {
    base_64: String,
}

impl FrameProcessor {
    pub fn from_base_64(base_64: String) -> Self {
        Self { base_64 }
    }

    pub fn as_base_64(&self) -> String {
        self.base_64.clone()
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        STANDARD.decode(&self.base_64).expect("Failed to decode base64!")
    }

    pub fn as_image_obj(&self) -> DynamicImage {
        image::load_from_memory(&self.as_bytes()).unwrap()
    }

    pub fn as_pixels(&self) -> Pixels {
        let image_obj = self.as_image_obj();
        let rgba = image_obj.to_rgba8();

        // This is specified in format like this:
        // [R, G, B, A, R, G, B, A, R, G, B, A, ...]
        let values = rgba.as_raw().clone();
        // Chunks is now an array of arrays, like this:
        // [[R, G, B, A], [R, G, B, A], ...]
        let chunks: Vec<&[u8]> = values.chunks_exact(4).collect();

        let mut pixels: Pixels = vec![];
        let mut x: u32 = 0;
        let mut y: u32 = 0;
        for chunk in chunks {
            if x >= image_obj.width() {
                x = 0;
                y += 1;
            }

            let pixel = Pixel {
                x: x.try_into().unwrap(),
                y: y.try_into().unwrap(),
                color: vec![chunk[0].into(), chunk[1].into(), chunk[2].into()],
            };
            pixels.push(pixel);

            x += 1;
        }

        pixels
    }

    pub fn set_to_base_64(&mut self, base_64: String) {
        self.base_64 = base_64;
    }

    pub fn set_to_bytes(&mut self, bytes: Vec<u8>) {
        let base_64 = STANDARD.encode(bytes);
        self.set_to_base_64(base_64);
    }

    pub fn set_to_image_obj(&mut self, image_obj: DynamicImage) {
        let mut bytes = Cursor::new(Vec::new());
        image_obj.write_to(&mut bytes, ImageFormat::Png).unwrap();
        self.set_to_bytes(bytes.into_inner());
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        let image = &self.as_image_obj();
        let resized = image.resize_exact(
            width,
            height,
            FilterType::Lanczos3,
        );
        self.set_to_image_obj(resized);
    }
}
