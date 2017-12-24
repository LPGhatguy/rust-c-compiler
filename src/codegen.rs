/// Generates x86 assembly from the AST

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

/// Generates an expression; everything outputs to %eax for simplicity.
fn generate_expression(expression: &AstExpression, output: &mut String) {
    match *expression {
        AstExpression::Constant { value } => {
            writeln!(output, "movl ${}, %eax", value).unwrap();
        },
        AstExpression::UnaryOperator { ref operator } => {
            match *operator.deref() {
                UnaryOperator::Negation { ref expression } => {
                    generate_expression(expression, output);
                    writeln!(output, "neg %eax").unwrap();
                },
                UnaryOperator::BitwiseComplement { ref expression } => {
                    generate_expression(expression, output);
                    writeln!(output, "not %eax").unwrap();
                },
                UnaryOperator::LogicalNegation { ref expression } => {
                    generate_expression(expression, output);
                    writeln!(output, "cmpl $0, %eax").unwrap();
                    writeln!(output, "movl $0, %eax").unwrap();
                    writeln!(output, "sete %al").unwrap();
                },
            }
        },
        AstExpression::BinaryOperator { ref operator } => {
            match *operator.deref() {
                BinaryOperator::Addition { ref a, ref b } => {
                    generate_expression(a, output);
                    writeln!(output, "push %eax").unwrap();
                    generate_expression(b, output);
                    writeln!(output, "pop %ecx").unwrap();
                    writeln!(output, "addl %ecx, %eax").unwrap();
                },
                BinaryOperator::Multiplication { ref a, ref b } => {
                    generate_expression(a, output);
                    writeln!(output, "push %eax").unwrap();
                    generate_expression(b, output);
                    writeln!(output, "pop %ecx").unwrap();
                    writeln!(output, "imul %ecx, %eax").unwrap();
                },
                BinaryOperator::Subtraction { ref a, ref b } => {
                    generate_expression(a, output);
                    writeln!(output, "push %eax").unwrap();
                    generate_expression(b, output);
                    writeln!(output, "movl %eax, %ecx").unwrap();
                    writeln!(output, "pop %eax").unwrap();
                    writeln!(output, "subl %ecx, %eax").unwrap();
                },
                BinaryOperator::Division { ref a, ref b } => {
                    // idivl x is:
                    // idivl [EDX:EAX] / x, stored in [EDX:EAX]
                    generate_expression(a, output);
                    writeln!(output, "push %eax").unwrap();
                    generate_expression(b, output);
                    writeln!(output, "movl %eax, %ecx").unwrap();
                    writeln!(output, "movl $0, %edx").unwrap();
                    writeln!(output, "pop %eax").unwrap();
                    writeln!(output, "idivl %ecx").unwrap();
                },
            }
        },
    }
}

fn generate_statement(statement: &AstStatement, output: &mut String) {
    match *statement {
        AstStatement::Return { ref expression } => {
            generate_expression(expression, output);
            writeln!(output, "ret").unwrap();
        },
    }
}

fn generate_function(function: &AstFunction, output: &mut String) {
    writeln!(output, ".globl {}", function.name).unwrap();
    writeln!(output, "{}:", function.name).unwrap();
    generate_statement(&function.statement, output);
}

pub fn generate_program(program: &AstProgram) -> String {
    let mut result = String::new();

    generate_function(&program.function, &mut result);

    result
}
