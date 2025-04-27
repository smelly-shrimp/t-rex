use clap::{arg, Parser};
use serde::{Deserialize, Serialize};

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
