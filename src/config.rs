use clap::{arg, Parser};

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(long, default_value_t = 16)]
    pub chunk: u32,

    #[arg(long, default_value_t = String::from("./assets/data.csv"))]
    pub csv: String,

    #[arg(long, default_value_t = String::from("./assets/image.png"))]
    pub asset: String,

    #[arg(long, default_value_t = String::from("./res"))]
    pub dest: String,

    #[arg(long, default_value_t = String::from(""))]
    pub pack: String,

	#[arg(long, default_value_t = String::from("./assets/structure.template.json"))]
    pub structure: String,
}
