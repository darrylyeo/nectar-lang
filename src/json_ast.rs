use super::parser;

pub fn parseToJSON<'a>(contents: &str) {
	match parser::parse(&contents) {
		Ok(parsed) => {
			match serde_json::to_string(&parsed) {
				Ok(json) => {
					println!("-> {}", json);
				},
				Err(error) => {
					println!("{}", error);
				}
			}
		},
		Err(error) => {
			println!("{}", error);
		}
	}
}