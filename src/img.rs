use image::{DynamicImage, GenericImageView, ImageBuffer, RgbaImage};

pub fn sub(img: &DynamicImage, sx: u32, sy: u32, chunk_size: u32) -> RgbaImage {
    let mut chunk_img: RgbaImage = ImageBuffer::new(chunk_size, chunk_size);

    for y in 0..chunk_size {
        for x in 0..chunk_size {
            let pixel = img.get_pixel(x + sx * chunk_size, y + sy * chunk_size);
            chunk_img.put_pixel(x, y, pixel);
        }
    }

    chunk_img
}
