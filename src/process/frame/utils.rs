use std::io::Cursor;
use image::{DynamicImage, ImageFormat};

pub fn get_image_bytes(img: &DynamicImage) -> image::ImageResult<usize> {
    let mut buf = Cursor::new(Vec::new());
    img.write_to(&mut buf, ImageFormat::Png)?;
    Ok(buf.get_ref().len())
}
