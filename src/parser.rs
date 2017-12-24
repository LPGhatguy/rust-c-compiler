use lexer::Token;

type TokenStream<'a> = &'a [Token<'a>];

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
pub enum UnaryOperator {
    Negation {
        expression: AstExpression,
    },
    BitwiseComplement {
        expression: AstExpression,
    },
    LogicalNegation {
        expression: AstExpression,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub enum AstExpression {
    Constant {
        value: u64,
    },
    UnaryOperator {
        operator: Box<UnaryOperator>,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub enum AstStatement {
    Return {
        expression: AstExpression
    },
}

fn simple_eat<'a>(
    tokens: TokenStream<'a>,
    eat_token: Token<'a>,
) -> Option<(TokenStream<'a>, &'a Token<'a>)> {
    match tokens.first() {
        Some(token) => {
            if *token == eat_token {
                Some((&tokens[1..], token))
            } else {
                None
            }
        }
        None => None,
    }
}

pub fn parse_program<'a>(tokens: TokenStream<'a>) -> Option<AstProgram<'a>> {
    match parse_function(tokens) {
        Some((_, function)) => Some(AstProgram { function }),
        None => None,
    }
}

fn parse_function<'a>(tokens: TokenStream<'a>) -> Option<(TokenStream<'a>, AstFunction<'a>)> {
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

    Some((tokens, AstFunction { name, statement }))
}

fn parse_statement<'a>(tokens: TokenStream<'a>) -> Option<(TokenStream<'a>, AstStatement)> {
    let (tokens, _) = simple_eat(tokens, Token::Keyword("return"))?;

    let (tokens, expression) = parse_expression(tokens)?;

    let (tokens, _) = simple_eat(tokens, Token::Semicolon)?;

    Some((tokens, AstStatement::Return { expression }))
}

fn parse_constant<'a>(tokens: TokenStream<'a>) -> Option<(TokenStream<'a>, AstExpression)> {
    let (tokens, value) = match tokens.first()? {
        &Token::IntegerLiteral(value) => (&tokens[1..], value),
        _ => return None,
    };

    Some((tokens, AstExpression::Constant { value }))
}

fn parse_unary_operator<'a>(tokens: TokenStream<'a>) -> Option<(TokenStream<'a>, UnaryOperator)> {
    let (tokens, operator) = match tokens.first()? {
        &Token::Operator(operator) => (&tokens[1..], operator),
        _ => return None,
    };

    let (tokens, expression) = parse_expression(tokens)?;

    let node = match operator {
        "~" => UnaryOperator::BitwiseComplement { expression },
        "!" => UnaryOperator::LogicalNegation { expression },
        "-" => UnaryOperator::Negation { expression },
        _ => unimplemented!(),
    };

    Some((tokens, node))
}

fn parse_expression<'a>(tokens: TokenStream<'a>) -> Option<(TokenStream<'a>, AstExpression)> {
    match parse_constant(tokens) {
        Some(result) => return Some(result),
        None => {},
    }

    match parse_unary_operator(tokens) {
        Some((tokens, operator)) => return Some((tokens, AstExpression::UnaryOperator {
            operator: Box::new(operator),
        })),
        None => {},
    }

    None
}
