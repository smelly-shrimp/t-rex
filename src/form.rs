use inquire::Text;

pub fn csv_path() -> String {
	let def = String::from("./assets/data.csv");

	Text::new("Enter CSV path")
		.with_default(&def)
		.prompt()
		.unwrap_or_else(|_| def)
}

pub fn asset_path() -> String {
	let def = String::from("./assets/image.png");

	Text::new("Enter asset path")
		.with_default(&def)
		.prompt()
		.unwrap_or_else(|_| def)
}

pub fn chunk_size() -> u32 {
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

pub fn is_pack() -> bool {
	let def = false;

	let res = Text::new("Do you want to create entire resource pack?")
		.with_default(&def.to_string())
		.prompt()
		.unwrap_or_else(|_| def.to_string());

	match res.parse::<bool>() {
		Ok(is_pack) => is_pack,
		Err(_) => {
			println!("Invalid pack value, using default ({})", def);
			def
		},
	}
}

pub fn dest_path() -> String {
	let def = String::from("./res");

	Text::new("Enter destination path")
		.with_default(&def)
		.prompt()
		.unwrap_or_else(|_| def)
}
