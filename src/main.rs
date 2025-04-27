use clap::Parser;

pub mod config;
pub mod data;
pub mod img;
pub mod utils;
pub mod form;

fn main() {
    let args = config::Args::parse();

    let data = if args.ui {
        println!("Hello! It's texture maker configuration.");

        if form::is_help() {
            println!("\n\nINFO! Help is not implemented yet.\n\n");
        }

        let structure_path = form::structure_path();
        let csv_path = form::csv_path();
        let asset_path = form::asset_path();

        let chunk_size = form::chunk_size();

        let pack_path = if form::is_pack() {
            let pack_path = form::pack_path();
            println!("INFO! The destination path will be inside {}", pack_path);

            pack_path
        } else {
            String::from("")
        };

        let dest_path = form::dest_path();

        if !form::is_confirm() {
            println!("INFO! Cancelling...");
            return;
        }

        config::Args {
            chunk: chunk_size,
            csv: csv_path,
            asset: asset_path,
            dest: dest_path,
            pack: pack_path,
            structure: structure_path,
            ui: args.ui,
        }
    } else {
        args
    };

    println!("{:?}", data);

    // let rows = data::read_rows(&args.csv);
    // let img = image::open(args.asset).unwrap();

    // let mut dest = args.dest;
    // if args.pack.len() > 0 {
    //     dest = format!("{}/", &args.pack);
    //     data::setup_structure(&args.structure, &dest);
    // }

    // for row in &rows {
    //     let chunk_img = img::sub(&img, row.x, row.y, args.chunk);

    //     let path = format!("{}/{}.png", &dest, &row.name);
    //     utils::fill_path(&path);
    //     chunk_img.save(&path).unwrap();
    // }
}
