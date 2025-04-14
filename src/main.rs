use csv::Reader;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Row {
    x: i32,
    y: i32,
    name: String,
}

fn read_rows(path: &str) -> Vec<Row> {
    let mut rd = Reader::from_path(path)
        .expect("no such file");

    let mut rows = Vec::<Row>::new();

    for res in rd.deserialize() {
        let row: Row = res.unwrap();
        rows.push(row);
    }

    rows
}

fn main() {
    let blocks = read_rows("src/blocks.csv");

    dbg!(blocks);
}
