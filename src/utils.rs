use std::{fs::{self, File}, io::BufWriter, path::Path};

use serde::Serialize;
use serde_json::ser::PrettyFormatter;

use crate::config::Args;

pub fn get_last_config() -> Args {
    let path = Path::new("./lc.json");
    
    if path.exists() {
        let data = fs::read_to_string(&path).unwrap();

        serde_json::from_str(&data).unwrap()
    } else {
        panic!("No last configuration found!");
    }
}

pub fn set_last_config(args: &Args) {
    let f = File::create("./lc.json").unwrap();
    let w = BufWriter::new(f);

    let ft = PrettyFormatter::with_indent(b"    ");
    let mut ser = serde_json::Serializer::with_formatter(w, ft);

    args.serialize(&mut ser).unwrap();
}
