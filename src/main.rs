pub mod data;
pub mod img;

use std::{fs, path::Path};
use clap::{arg, Parser};

#[derive(Parser, Debug)]
struct Args {
    #[arg(long, default_value_t = 16)]
    chunk: u32,
    #[arg(long, default_value_t = String::from("./assets/data.csv"))]
    csv: String,
    #[arg(long, default_value_t = String::from("./assets/image.png"))]
    asset: String,
    #[arg(long, default_value_t = String::from("./res"))]
    dir: String,
    #[arg(long, default_value_t = String::from(""))]
    pack: String,
}

fn fill_path(path_str: &String) {
    let path = Path::new(&path_str);

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
}

fn main() {
    let args = Args::parse();

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
