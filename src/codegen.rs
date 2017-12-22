use std::fmt::Write;

use parser::{AstProgram, AstFunction, AstExpression, AstStatement};

fn generate_statement(statement: &AstStatement, output: &mut String) {
	match statement {
		&AstStatement::Return { ref expression } => {
			write!(output, "movl ${}, %eax\nret", expression.value);
		},
	}
}

fn generate_function(function: &AstFunction, output: &mut String) {
	write!(output, ".globl {}\n{}:\n", function.name, function.name);
	generate_statement(&function.statement, output);
}

pub fn generate_program(program: &AstProgram) -> String {
	let mut result = String::new();

	generate_function(&program.function, &mut result);

	result
}
