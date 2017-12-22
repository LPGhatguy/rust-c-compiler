use parser::{AstProgram, AstFunction, AstExpression, AstStatement};

fn generate_statement(statement: &AstStatement) -> String {
	match statement {
		&AstStatement::Return { ref expression } => {
			format!("movl ${}, %eax\nret", expression.value)
		},
	}
}

fn generate_function(function: &AstFunction) -> String {
	format!(".globl {}\n{}:\n{}", function.name, function.name, generate_statement(&function.statement))
}

pub fn generate_program(program: &AstProgram) -> String {
	generate_function(&program.function)
}
