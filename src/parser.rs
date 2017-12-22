use lexer::Token;

#[derive(Debug, PartialEq, Eq)]
pub struct AstProgram<'a> {
	pub function: AstFunction<'a>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct AstFunction<'a> {
	pub name: &'a str,
	pub statement: AstStatement,
}

#[derive(Debug, PartialEq, Eq)]
pub struct AstExpression {
	pub value: u64,
}

#[derive(Debug, PartialEq, Eq)]
pub enum AstStatement {
	Return(AstExpression),
}

pub fn parse_program<'a>(tokens: &'a [Token<'a>]) -> Option<AstProgram<'a>> {
	match parse_function(tokens) {
		Some((function, _)) => Some(AstProgram { function }),
		None => None,
	}
}

fn parse_function<'a>(tokens: &'a [Token<'a>]) -> Option<(AstFunction<'a>, &'a [Token<'a>])> {
	let tokens = match tokens.first() {
		Some(token) => match token {
			&Token::Keyword("int") => &tokens[1..],
			_ => return None,
		},
		None => return None,
	};

	let (tokens, func_name) = match tokens.first() {
		Some(token) => match token {
			&Token::Identifier(name) => (&tokens[1..], name),
			_ => return None,
		},
		None => return None,
	};

	None
}