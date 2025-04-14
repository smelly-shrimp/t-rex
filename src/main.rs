use csv::Reader;
use image::{DynamicImage, GenericImageView, ImageBuffer, RgbaImage};
use serde::Deserialize;
use std::{fs, path::Path};
use clap::{arg, Parser};

#[derive(Debug, Deserialize)]
struct Row {
    x: u32,
    y: u32,
    name: String,
}

#[derive(Parser, Debug)]
struct Args {
    #[arg(long, default_value_t = 16)]
    chunk: u32,
    #[arg(long, default_value_t = String::from("./data.csv"))]
    csv: String,
    #[arg(long, default_value_t = String::from("./image.png"))]
    asset: String,
    #[arg(long, default_value_t = String::from("./blocks"))]
    dir: String,
}

fn read_rows(path: &str) -> Vec<Row> {
    let mut rd = Reader::from_path(path).unwrap();

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

fn fill_path(path_str: &String) {
    let path = Path::new(&path_str);

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
}

fn main() {
    let args = Args::parse();

    let rows = read_rows(&args.csv);
    let img = image::open(args.asset).unwrap();

    for row in &rows {
        let chunk_img = sub_img(&img, row.x, row.y, args.chunk);

        let path = format!("{}/{}.png", args.dir, row.name);
        fill_path(&path);
        chunk_img.save(&path).unwrap();
    }
}
