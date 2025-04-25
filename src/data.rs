use std::{fs::{self, File}, io::Write};

use csv::Reader;
use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct Row {
    pub x: u32,
    pub y: u32,
    pub name: String,
}

pub fn read_rows(path: &str) -> Vec<Row> {
    let mut rd = Reader::from_path(path).unwrap();

    let mut rows = Vec::<Row>::new();

    for res in rd.deserialize() {
        let row: Row = res.unwrap();
        rows.push(row);
    }

    rows
}

fn build_structure(value: &Value, path: &str) {
    if let Value::Object(dir) = value {
        for (k, v) in dir {
            if let Value::String(content) = v {
                let p = format!("{}{}", path, k);

                let mut f = File::create(&p).unwrap();
                f.write_all(content.as_bytes()).unwrap();
            } else {
                let p = format!("{}{}/", &path, k);

                fs::create_dir(&p).unwrap();
                build_structure(v, &p);
            }
        }
    }
}

pub fn setup_structure(structure_path: &str, pack_path: &str) {
    let data = fs::read_to_string(&structure_path).unwrap();

    let json: Value = serde_json::from_str(&data).unwrap();

    build_structure(&json, &pack_path);
}
