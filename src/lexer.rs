use regex::Regex;

#[derive(Debug, PartialEq, Eq)]
pub enum Token<'a> {
	Whitespace(&'a str),
	Keyword(&'a str),
	OpenBrace,
	CloseBrace,
	OpenParen,
	CloseParen,
	Semicolon,
	Identifier(&'a str),
	IntegerLiteral(u64),
}

fn try_advance<'a, F>(source: &'a str, pattern: &Regex, tokenizer: F) -> Option<(&'a str, Token<'a>)>
where F: Fn(&'a str) -> Token<'a>
{
	if let Some(range) = pattern.find(source) {
		let contents = &source[range.start() .. range.end()];
		Some((&source[range.end()..], tokenizer(contents)))
	} else {
		None
	}
}

#[test]
fn test_try_advance() {
	let pattern = Regex::new(r"^\w+").unwrap();
	let source = "hello world foo";

	let (rest, token) = try_advance(source, pattern, |s| { Token::Keyword(s) })
		.expect("Unable to advance pattern!");

	assert_eq!(rest, " world foo");
	assert_eq!(token, Token::Keyword("hello"));
}

pub fn lex<'a>(source: &'a str) -> Vec<Token<'a>> {
	lazy_static! {
		static ref PATTERN_WHITESPACE: Regex = Regex::new(r"^\s+").unwrap();
		static ref PATTERN_KEYWORD: Regex = Regex::new(r"^(int|return)").unwrap();
		static ref PATTERN_OPEN_BRACE: Regex = Regex::new(r"^\{").unwrap();
		static ref PATTERN_CLOSE_BRACE: Regex = Regex::new(r"^\}").unwrap();
		static ref PATTERN_OPEN_PAREN: Regex = Regex::new(r"^\(").unwrap();
		static ref PATTERN_CLOSE_PAREN: Regex = Regex::new(r"^\)").unwrap();
		static ref PATTERN_SEMICOLON: Regex = Regex::new(r"^;").unwrap();
		static ref PATTERN_IDENTIFIER: Regex = Regex::new(r"^[a-zA-Z]\w*").unwrap();
		static ref PATTERN_INTEGER_LITERAL: Regex = Regex::new(r"^[0-9]+").unwrap();
	}

	let mut tokens = Vec::new();
	let mut current = source;

	loop {
		let result = try_advance(current, &PATTERN_WHITESPACE, |s| Token::Whitespace(s))
			.or_else(|| try_advance(current, &PATTERN_KEYWORD, |s| Token::Keyword(s)))
			.or_else(|| try_advance(current, &PATTERN_OPEN_BRACE, |_| Token::OpenBrace))
			.or_else(|| try_advance(current, &PATTERN_CLOSE_BRACE, |_| Token::CloseBrace))
			.or_else(|| try_advance(current, &PATTERN_OPEN_PAREN, |_| Token::OpenParen))
			.or_else(|| try_advance(current, &PATTERN_CLOSE_PAREN, |_| Token::CloseParen))
			.or_else(|| try_advance(current, &PATTERN_SEMICOLON, |_| Token::Semicolon))
			.or_else(|| try_advance(current, &PATTERN_IDENTIFIER, |s| Token::Identifier(s)))
			.or_else(|| try_advance(current, &PATTERN_INTEGER_LITERAL, |s| Token::IntegerLiteral(s.parse().unwrap())));

		match result {
			Some((next_current, token)) => {
				current = next_current;

				// Skip whitespace!
				// If we want a style-preserving lexer/parser in the future,
				// that'll be pretty easy.
				match token {
					Token::Whitespace(_) => {},
					_ => tokens.push(token),
				}
			},
			None => break,
		}
	}

	if !current.is_empty() {
		eprintln!("Unknown garbage at {:?}", current);
	}

	tokens
}