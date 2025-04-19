pub mod config;
pub mod data;
pub mod img;

use clap::Parser;
use std::{fs, path::Path};

fn fill_path(path_str: &String) {
    let path = Path::new(&path_str);

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
}

fn main() {
    let args = config::Args::parse();

    let rows = data::read_rows(&args.csv);
    let img = image::open(args.asset).unwrap();

    if args.pack.len() > 0 {
        data::setup_structure("./assets/structure.template.json");
    }

    for row in &rows {
        let chunk_img = img::sub(&img, row.x, row.y, args.chunk);

        let path = format!("{}/{}.png", &args.dir, &row.name);
        fill_path(&path);
        chunk_img.save(&path).unwrap();
    }
}
