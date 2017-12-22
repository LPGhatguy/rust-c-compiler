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
	Return {
		expression: AstExpression,
	},
}

fn simple_eat<'a>(tokens: &'a [Token<'a>], eat_token: Token<'a>) -> Option<(&'a [Token<'a>], &'a Token<'a>)> {
	match tokens.first() {
		Some(token) => {
			if *token == eat_token {
				Some((&tokens[1..], token))
			} else {
				None
			}
		},
		None => None,
	}
}

pub fn parse_program<'a>(tokens: &'a [Token<'a>]) -> Option<AstProgram<'a>> {
	match parse_function(tokens) {
		Some((_, function)) => Some(AstProgram { function }),
		None => None,
	}
}

fn parse_function<'a>(tokens: &'a [Token<'a>]) -> Option<(&'a [Token<'a>], AstFunction<'a>)> {
	let (tokens, _) = simple_eat(tokens, Token::Keyword("int"))?;

	let (tokens, name) = match tokens.first()? {
		&Token::Identifier(name) => (&tokens[1..], name),
		_ => return None,
	};

	let (tokens, _) = simple_eat(tokens, Token::OpenParen)?;
	let (tokens, _) = simple_eat(tokens, Token::CloseParen)?;
	let (tokens, _) = simple_eat(tokens, Token::OpenBrace)?;

	let (tokens, statement) = parse_statement(tokens)?;

	let (tokens, _) = simple_eat(tokens, Token::CloseBrace)?;

	Some((tokens, AstFunction {
		name,
		statement,
	}))
}

fn parse_statement<'a>(tokens: &'a [Token<'a>]) -> Option<(&'a [Token<'a>], AstStatement)> {
	let (tokens, _) = simple_eat(tokens, Token::Keyword("return"))?;

	let (tokens, expression) = parse_expression(tokens)?;

	let (tokens, _) = simple_eat(tokens, Token::Semicolon)?;

	Some((tokens, AstStatement::Return {
		expression,
	}))
}

fn parse_expression<'a>(tokens: &'a [Token<'a>]) -> Option<(&'a [Token<'a>], AstExpression)> {
	let (tokens, value) = match tokens.first()? {
		&Token::IntegerLiteral(value) => (&tokens[1..], value),
		_ => return None,
	};

	Some((tokens, AstExpression {
		value,
	}))
}