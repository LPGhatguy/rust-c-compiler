#[macro_use]
extern crate lazy_static;
extern crate regex;

mod lexer;
mod parser;
mod codegen;

fn main() {
    let source = r"
		int main() {
			return ~-!~!3;
		}
	";

    let tokens = lexer::lex(source);

    let ast = parser::parse_program(&tokens).expect("Could not parse program!");

    let asm = codegen::generate_program(&ast);

    eprintln!("Tokens:\n{:?}\n", tokens);
    eprintln!("AST:\n{:?}\n", ast);
    eprintln!("ASM:\n{}\n", asm);

    println!("{}\n", asm);
}
