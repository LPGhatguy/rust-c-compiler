use std::fmt::Write;
use std::ops::Deref;

use parser::{
	AstExpression,
	AstFunction,
	AstProgram,
	AstStatement,
	BinaryOperator,
	UnaryOperator,
};

fn generate_expression(expression: &AstExpression, output: &mut String) {
	match *expression {
		AstExpression::Constant { value } => {
			write!(output, "movl ${}, %eax\n", value).unwrap();
		},
		AstExpression::UnaryOperator { ref operator } => {
			match *operator.deref() {
				UnaryOperator::Negation { ref expression } => {
					generate_expression(expression, output);
					write!(output, "neg %eax\n").unwrap();
				},
				UnaryOperator::BitwiseComplement { ref expression } => {
					generate_expression(expression, output);
					write!(output, "not %eax\n").unwrap();
				},
				UnaryOperator::LogicalNegation { ref expression } => {
					generate_expression(expression, output);
					write!(output, "cmpl $0, %eax\n").unwrap();
					write!(output, "movl $0, %eax\n").unwrap();
					write!(output, "sete %al\n").unwrap();
				},
			}
		},
		AstExpression::BinaryOperator { ref operator } => {
			match *operator.deref() {
				BinaryOperator::Addition { ref a, ref b } => {
					generate_expression(a, output);
					write!(output, "push %eax\n").unwrap();
					generate_expression(b, output);
					write!(output, "pop %ecx\n").unwrap();
					write!(output, "addl %ecx, %eax\n").unwrap();
				},
				_ => unimplemented!(),
			}
		},
	}
}

fn generate_statement(statement: &AstStatement, output: &mut String) {
	match *statement {
		AstStatement::Return { ref expression } => {
			generate_expression(expression, output);
			write!(output, "ret\n").unwrap();
		},
	}
}

fn generate_function(function: &AstFunction, output: &mut String) {
	write!(output, ".globl {}\n{}:\n", function.name, function.name).unwrap();
	generate_statement(&function.statement, output);
}

pub fn generate_program(program: &AstProgram) -> String {
	let mut result = String::new();

	generate_function(&program.function, &mut result);

	result
}
