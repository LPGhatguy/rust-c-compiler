/// Hand-written recurisive-descent parser
///
/// I'm not satisfied with its cleanliness -- I'll probably either refactor it
/// until the end of time or just rewrite it as a different kind of parser.
///
/// The AST representation is kind of awkward, with boxing happening at the
/// operator level, and lots of repetition.

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
pub enum BinaryOperator {
    Addition {
        a: AstExpression,
        b: AstExpression,
    },
    Subtraction {
        a: AstExpression,
        b: AstExpression,
    },
    Multiplication {
        a: AstExpression,
        b: AstExpression,
    },
    Division {
        a: AstExpression,
        b: AstExpression,
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
    BinaryOperator {
        operator: Box<BinaryOperator>,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub enum AstStatement {
    Return {
        expression: AstExpression
    },
}

/// Tries to consume a token by equality, yielding that token and the rest of
/// the token stream.
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

/// <program> ::= <function>
pub fn parse_program<'a>(tokens: TokenStream<'a>) -> Option<AstProgram<'a>> {
    match parse_function(tokens) {
        Some((_, function)) => Some(AstProgram { function }),
        None => None,
    }
}

/// <function> ::= "int" <identifier> "(" ")" "{" <statement> "}"
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

/// <statement> ::= "return" <expression> ";"
fn parse_statement<'a>(tokens: TokenStream<'a>) -> Option<(TokenStream<'a>, AstStatement)> {
    let (tokens, _) = simple_eat(tokens, Token::Keyword("return"))?;

    let (tokens, expression) = parse_expression(tokens)?;

    let (tokens, _) = simple_eat(tokens, Token::Semicolon)?;

    Some((tokens, AstStatement::Return { expression }))
}

/// <constant> (terminal)
fn parse_constant<'a>(tokens: TokenStream<'a>) -> Option<(TokenStream<'a>, AstExpression)> {
    let (tokens, value) = match tokens.first()? {
        &Token::IntegerLiteral(value) => (&tokens[1..], value),
        _ => return None,
    };

    Some((tokens, AstExpression::Constant { value }))
}

/// <unary_expression> ::= ("~" | "!" | "-") <factor>
fn parse_unary_expression<'a>(tokens: TokenStream<'a>) -> Option<(TokenStream<'a>, AstExpression)> {
    let (tokens, operator) = match tokens.first()? {
        &Token::Operator(operator) => (&tokens[1..], operator),
        _ => return None,
    };

    let (tokens, expression) = parse_factor(tokens)?;

    let node = match operator {
        "~" => UnaryOperator::BitwiseComplement { expression },
        "!" => UnaryOperator::LogicalNegation { expression },
        "-" => UnaryOperator::Negation { expression },
        _ => unimplemented!(),
    };

    Some((tokens, AstExpression::UnaryOperator {
        operator: Box::new(node),
    }))
}

/// <paren_expression> ::= "(" <expression> ")"
fn parse_paren_expression<'a>(tokens: TokenStream<'a>) -> Option<(TokenStream<'a>, AstExpression)> {
    let (tokens, _) = simple_eat(tokens, Token::OpenParen)?;
    let (tokens, expression) = parse_expression(tokens)?;
    let (tokens, _) = simple_eat(tokens, Token::CloseParen)?;

    Some((tokens, expression))
}

/// <factor> ::= <paren_expression> | <unary_expression> | <constant>
fn parse_factor<'a>(tokens: TokenStream<'a>) -> Option<(TokenStream<'a>, AstExpression)> {
    parse_paren_expression(tokens)
        .or_else(|| parse_unary_expression(tokens))
        .or_else(|| parse_constant(tokens))
}

/// <term> ::= <factor> { ("*" | "/") <factor> }
fn parse_term<'a>(tokens: TokenStream<'a>) -> Option<(TokenStream<'a>, AstExpression)> {
    let (mut tokens, mut factor) = parse_factor(tokens)?;

    loop {
        let operator = match tokens.first() {
            Some(operator) => {
                match operator {
                    &Token::Operator("*") | &Token::Operator("/") => {
                        tokens = &tokens[1..];
                        operator
                    },
                    _ => break,
                }
            },
            None => break,
        };

        match operator {
            &Token::Operator(operator_kind) => {
                let second_factor = match parse_factor(tokens) {
                    Some((next_tokens, second_factor)) => {
                        tokens = next_tokens;
                        second_factor
                    },
                    None => return None,
                };

                let inner = match operator_kind {
                    "*" => BinaryOperator::Multiplication {
                        a: factor,
                        b: second_factor,
                    },
                    "/" => BinaryOperator::Division {
                        a: factor,
                        b: second_factor,
                    },
                    _ => break,
                };

                factor = AstExpression::BinaryOperator {
                    operator: Box::new(inner),
                };
            },
            _ => break,
        }
    }

    Some((tokens, factor))
}

/// <expression> ::= <term> { ("+" | "-") <term> }
fn parse_expression<'a>(tokens: TokenStream<'a>) -> Option<(TokenStream<'a>, AstExpression)> {
    let (mut tokens, mut term) = parse_term(tokens)?;

    loop {
        let operator = match tokens.first() {
            Some(operator) => {
                match operator {
                    &Token::Operator("+") | &Token::Operator("-") => {
                        tokens = &tokens[1..];
                        operator
                    },
                    _ => break,
                }
            },
            None => break,
        };

        match operator {
            &Token::Operator(operator_kind) => {
                let second_term = match parse_term(tokens) {
                    Some((next_tokens, second_term)) => {
                        tokens = next_tokens;
                        second_term
                    },
                    None => return None,
                };

                let inner = match operator_kind {
                    "+" => BinaryOperator::Addition {
                        a: term,
                        b: second_term,
                    },
                    "-" => BinaryOperator::Subtraction {
                        a: term,
                        b: second_term,
                    },
                    _ => break,
                };

                term = AstExpression::BinaryOperator {
                    operator: Box::new(inner),
                };
            },
            _ => break,
        }
    }

    Some((tokens, term))
}
