use std::{env, fs, io::{self, BufRead, Write}};

extern crate pest;
extern crate pest_consume;
extern crate serde;
extern crate serde_json;

mod types;
mod parser;
// mod interpreter;
mod json_ast;


fn print(string: &str){
	print!("{}", string);
    io::stdout().flush().unwrap();
}


fn repl() {
	println!("Welcome to the Nectar REPL! You're using version {}.", env!("CARGO_PKG_VERSION"));
	println!("{}", "Type statements like this: @Nectar is a #language.\n");

	let stdin = io::stdin();
	print("nectar $ ");
	for line in stdin.lock().lines() {
		// interpreter::eval(&line.unwrap());
		json_ast::parseToJSON(&line.unwrap());
		print("\nnectar $ ");
	}
}


fn main() {
	let args: Vec<String> = env::args().collect();
	match args.len() - 1 {
		0 => {
			repl()
		}
		1 => {
			let filename = &args[1];
			let contents = fs::read_to_string(filename)
				.expect("Couldn't read the file.");
			// interpreter::eval(&contents)
			json_ast::parseToJSON(&contents)
		}
		_ => {}
	}
}