pub mod config;
pub mod data;
pub mod img;
pub mod utils;
pub mod form;

fn main() {
    println!("Hello! It's texture maker configuration.");

    form::get_csv_path();
    form::get_asset_path();
    
    form::get_chunk_size();

    form::get_dest_path();

    // let args = config::Args::parse();

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
