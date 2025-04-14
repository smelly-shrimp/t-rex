use csv::Reader;
use image::{DynamicImage, GenericImageView, ImageBuffer, ImageResult, RgbaImage};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Row {
    x: u32,
    y: u32,
    name: String,
}

fn read_rows(path: &str) -> Vec<Row> {
    let mut rd = Reader::from_path(path)
        .expect("no such file");

    let mut rows = Vec::<Row>::new();

    for res in rd.deserialize() {
        let row: Row = res.unwrap();
        rows.push(row);
    }

    rows
}

fn sub_img(img: &DynamicImage, sx: u32, sy: u32, chunk_size: u32) -> RgbaImage {
    let mut chunk_img: RgbaImage = ImageBuffer::new(chunk_size, chunk_size);

    for y in 0..chunk_size {
        for x in 0..chunk_size {
            let pixel = img.get_pixel(x + sx * chunk_size, y + sy * chunk_size);
            chunk_img.put_pixel(x, y, pixel);
        }
    }

    chunk_img
}

fn main() {
    let chunk_size = 16;
    let blocks = read_rows("src/blocks.csv");

    let img = image::open("src/blocks.png")
        .expect("file doesn't exist");

    for block in &blocks {
        let chunk_img = sub_img(&img, block.x, block.y, chunk_size);

        chunk_img.save(format!("src/blocks/{}.png", block.name)).unwrap();
    }
}
