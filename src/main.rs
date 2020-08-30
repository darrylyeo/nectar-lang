use std::{env, fs, io::{self, BufRead, Write}};

extern crate pest;
extern crate pest_consume;

mod types;
mod parser;
mod interpreter;


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
		interpreter::eval(&line.unwrap());
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
			interpreter::eval(&contents)
		}
		_ => {}
	}
}