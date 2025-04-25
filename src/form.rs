use inquire::Text;

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
