use std::{env, fs, io::{self, BufRead, Write}};

extern crate pest;
extern crate pest_consume;

mod types;
mod parser;


fn print(string: &str){
	print!("{}", string);
    io::stdout().flush().unwrap();
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}


fn eval(contents: &str) {
	match parser::parse(&contents) {
		Ok(parsed) => {
			for statement in parsed {
				println!("-> {:?}", statement);
				print_type_of(&statement);
			}
		},
		Err(error) => {
			println!("{}", error);
		}
	}
}


fn repl() {
	println!("Welcome to the Nectar REPL! You're using version {}.", env!("CARGO_PKG_VERSION"));
	println!("{}", "Type statements like this: @Nectar is a #language.\n");

	let stdin = io::stdin();
	print("nectar $ ");
	for line in stdin.lock().lines() {
		eval(&line.unwrap());
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
			eval(&contents)
		}
		_ => {}
	}
}