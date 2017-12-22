#[macro_use]
extern crate lazy_static;
extern crate regex;

mod lexer;
mod parser;

fn main() {
   let source = r"
		int main() {
			return 2;
		}
	";

	let tokens = lexer::lex(source);

	println!("{:?}", tokens);
}
