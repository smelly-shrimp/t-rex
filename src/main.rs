use csv::Reader;
use image::{ImageBuffer, RgbaImage};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Row {
    x: i32,
    y: i32,
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

fn main() {
    let chunk_size = 16;
    let blocks = read_rows("src/blocks.csv");

    for block in &blocks {
        let chunk_img: RgbaImage = ImageBuffer::new(chunk_size, chunk_size);

        chunk_img.save(format!("src/blocks/{}.png", block.name)).unwrap();
    }
}
