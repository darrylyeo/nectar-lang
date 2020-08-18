mod parser;

extern crate pest;
// #[macro_use]
// extern crate pest_derive;
extern crate pest_consume;

use std::{env, fs, io::{self, BufRead, Write}};


fn print(string: &str){
	print!("{}", string);
    io::stdout().flush().unwrap();
}

fn main() {
	let args: Vec<String> = env::args().collect();
	match args.len() - 1 {
		0 => {
			let stdin = io::stdin();
			print("nectar $ ");
			for line in stdin.lock().lines() {
				let contents = line.unwrap();

				let parsed = parser::parse(&contents);
				println!("{:?}", parsed);

				print("\nnectar $ ");
			}
		}
		1 => {
			let filename = &args[1];

			let contents = fs::read_to_string(filename)
				.expect("Couldn't read the file");

			let parsed = parser::parse(&contents);
			println!("{:?}", parsed);
		}
		_ => {}
	}
}