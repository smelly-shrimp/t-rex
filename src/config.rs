use std::{fs::{self, File}, io::BufWriter, path::Path};

use clap::{arg, Parser};
use serde::{Deserialize, Serialize};
use serde_json::ser::PrettyFormatter;

#[derive(Parser, Debug, Deserialize, Serialize)]
pub struct Args {
    #[clap(short, long)]
    pub last: bool,

    #[clap(short, long)]
    pub ui: bool,

    #[arg(long, default_value_t = String::from("./assets/structure.template.json"))]
    pub structure: String,

    #[arg(long, default_value_t = String::from("./assets/data.csv"))]
    pub csv: String,

    #[arg(long, default_value_t = String::from("./assets/image.png"))]
    pub asset: String,

    #[arg(long, default_value_t = 16)]
    pub chunk: u32,

    #[arg(long, default_value_t = String::from("./res_pack"))]
    pub pack: String,

    #[arg(long, default_value_t = String::from("./res"))]
    pub dest: String,
}

pub fn get_last() -> Args {
    let path = Path::new("./lc.json");
    
    if path.exists() {
        let data = fs::read_to_string(&path).unwrap();

        serde_json::from_str(&data).unwrap()
    } else {
        panic!("No last configuration found!");
    }
}

pub fn save_last(args: &Args) {
    let f = File::create("./lc.json").unwrap();
    let w = BufWriter::new(f);

    let ft = PrettyFormatter::with_indent(b"    ");
    let mut ser = serde_json::Serializer::with_formatter(w, ft);

    args.serialize(&mut ser).unwrap();
}
