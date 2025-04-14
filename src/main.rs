use csv::Reader;
use image::{DynamicImage, GenericImageView, ImageBuffer, RgbaImage};
use serde::Deserialize;
use std::io::{self, Write};

#[derive(Debug, Deserialize)]
struct Row {
    x: u32,
    y: u32,
    name: String,
}

struct InputVals {
    chunk_size: u32,
    rows_path: String,
    img_path: String,
    dir_path: String,
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

fn read_input<T: std::str::FromStr>(default: T) -> T {
    let mut input = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();

    match input.trim().parse::<T>() {
        Ok(val) => val,
        Err(_) => default,
    }
}

fn read_defaults() -> InputVals {
    print!("chunk/block size (default: 16): ");
    let chunk_size = read_input(16);

    print!("path to csv data (default: ./data.csv): ");
    let rows_path = read_input(String::from("./data.csv"));

    print!("path to asset image (default: ./image.png): ");
    let img_path = read_input(String::from("./image.png"));

    print!("path to destination directory (default: blocks):");
    let dir_path = read_input(String::from("blocks"));

    InputVals { chunk_size, rows_path, img_path, dir_path }
}

fn main() {
    let input_vals = read_defaults();

    let rows = read_rows(&input_vals.rows_path);

    let img = image::open(input_vals.img_path)
        .expect("file doesn't exist");

    for row in &rows {
        let chunk_img = sub_img(&img, row.x, row.y, input_vals.chunk_size);

        chunk_img.save(format!("{}/{}.png", input_vals.dir_path, row.name)).unwrap();
    }
}
