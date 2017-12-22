#[macro_use]
extern crate lazy_static;
extern crate regex;

mod lexer;
mod parser;
mod codegen;

fn main() {
   let source = r"
		int main() {
			return 2;
		}
	";

	let tokens = lexer::lex(source);

	let ast = parser::parse_program(&tokens)
		.expect("Could not parse program!");

	let asm = codegen::generate(&ast);

	println!("{:?}\n", tokens);
	println!("{:?}\n", ast);
	println!("{:?}\n", asm);
}
