use csv::Reader;
use serde::Deserialize;

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
