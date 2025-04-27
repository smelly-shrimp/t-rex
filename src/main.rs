use clap::Parser;

pub mod config;
pub mod data;
pub mod img;
pub mod utils;
pub mod form;

fn main() {
    let args = config::Args::parse();

    let data = if args.last {
        println!("\n\nINFO! Running with previous configuration...\n\n");

        utils::get_last_config()
    } else if args.ui {
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

        let dest_path = form::dest_path(&args.structure);

        if !form::is_confirm() {
            println!("INFO! Cancelling...");
            return;
        }

        config::Args {
            last: args.last,
            ui: args.ui,
            structure: structure_path,
            csv: csv_path,
            asset: asset_path,
            chunk: chunk_size,
            pack: pack_path,
            dest: dest_path,
        }
    } else {
        args
    };

    utils::set_last_config(&data);

    let rows = data::read_rows(&data.csv);
    let img = image::open(&data.asset).unwrap();

    let dest = if data.pack.len() > 0 {
        let d = format!("{}/{}", &data.pack, &data.dest);
        data::setup_structure(&data.structure, &format!("{}/", &data.pack));

        d
    } else {
        data.dest.clone()
    };

    for row in &rows {
        let chunk_img = img::sub(&img, row.x, row.y, data.chunk);

        let path = format!("{}/{}.png", &dest, &row.name);

        utils::fill_path(&path);
        chunk_img.save(&path).unwrap();
    }
}
