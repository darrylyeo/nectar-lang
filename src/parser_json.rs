use wasm_bindgen::prelude::*;

use super::parser;


#[wasm_bindgen]
pub fn parse_to_json(contents: &str) -> String {
	match parser::parse(&contents) {
		Ok(parsed) =>
			match serde_json::to_string(&parsed) {
				Ok(json) =>
					json,

				Err(error) => serde_json::json!({
					"type": "error",
					"message": error.to_string()
				}).to_string()
			},

		Err(error) =>
			error.to_string()
	}
}