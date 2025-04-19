pub mod config;
pub mod data;
pub mod img;
pub mod utils;

use clap::Parser;

fn main() {
    let args = config::Args::parse();

    let rows = data::read_rows(&args.csv);
    let img = image::open(args.asset).unwrap();

    if args.pack.len() > 0 {
        data::setup_structure(&args.structure);
    }

    for row in &rows {
        let chunk_img = img::sub(&img, row.x, row.y, args.chunk);

        let path = format!("{}/{}.png", &args.dir, &row.name);
        utils::fill_path(&path);
        chunk_img.save(&path).unwrap();
    }
}
