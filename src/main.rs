#[macro_use]
extern crate lazy_static;
extern crate regex;

mod lexer;
mod parser;
mod codegen;

fn main() {
    let source = r"
        int main() {
            return 3 * (3 + 5 * 2);
        }
    ";

    let tokens = lexer::lex(source);

    eprintln!("Tokens:\n{:?}\n", tokens);

    let ast = parser::parse_program(&tokens).expect("Could not parse program!");

    eprintln!("AST:\n{:?}\n", ast);

    let asm = codegen::generate_program(&ast);

    eprintln!("ASM:\n{}\n", asm);

    println!("{}\n", asm);
}
