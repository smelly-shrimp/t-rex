use inquire::Text;

pub fn get_csv_path() -> String {
	let def = String::from("./assets/data.csv");

	let res = Text::new("Enter CSV path")
		.with_default(&def)
		.prompt()
		.unwrap_or_else(|_| def);

	res
}

pub fn get_asset_path() -> String {
	let def = String::from("./assets/image.png");

	let res = Text::new("Enter asset path")
		.with_default(&def)
		.prompt()
		.unwrap_or_else(|_| def);

	res
}

pub fn get_dest_path() -> String {
	let def = String::from("./res");

	let res = Text::new("Enter destination path")
		.with_default(&def)
		.prompt()
		.unwrap_or_else(|_| def);

	res
}

pub fn get_chunk_size() -> u32 {
	let def = 16;

	let res = Text::new("Enter chunk size")
		.with_default(&def.to_string())
		.prompt()
		.unwrap_or_else(|_| def.to_string());

	let chunk_size = match res.parse::<u32>() {
		Ok(size) => size,
		Err(_) => {
			println!("Invalid chunk size, using default ({})", def);
			def
		},
	};

	chunk_size
}
