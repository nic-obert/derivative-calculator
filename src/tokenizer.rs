use core::fmt;
use std::rc::Rc;

use regex::Regex;
use lazy_static::lazy_static;

use crate::errors;
use crate::ast::{UnparsedTree, Priority, FunctionTree};
use crate::functions::Functions;


lazy_static! {

    static ref TOKEN_REGEX: Regex = Regex::new(
        r#"(?m)[_a-zA-Z]\w*|-?\d+[.]\d*|-?[.]?\d+|[-+/*^()]|\S"#
    ).expect("Regex failed to compile");

    static ref VARIABLE_REGEX: Regex = Regex::new(
        r#"^[_a-zA-Z]\w*\z"#
    ).expect("Regex failed to compile");

}


pub struct SourceToken<'a> {
    pub string: &'a str,
    pub column: usize
}


pub enum TokenValue<'a> {
    Plus,
    Minus,
    Mul,
    Div,
    Pow,
    ParenOpen,
    ParenClose,
    Identifier (&'a str),
    Number(f64),
    Function(Functions)
}

impl fmt::Display for TokenValue<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenValue::Plus => write!(f, "+"),
            TokenValue::Minus => write!(f, "-"),
            TokenValue::Mul => write!(f, "*"),
            TokenValue::Div => write!(f, "/"),
            TokenValue::Pow => write!(f, "^"),
            TokenValue::ParenOpen => write!(f, "("),
            TokenValue::ParenClose => write!(f, ")"),
            TokenValue::Identifier(name) => write!(f, "{}", name),
            TokenValue::Number(n) => write!(f, "{}", n),
            TokenValue::Function(func) => write!(f, "{}", func)
        }
    }
}

impl TokenValue<'_> {

    pub fn base_priority(&self) -> Priority {
        match self {
            TokenValue::ParenClose => 0, // Doesn't get evaluated
            TokenValue::Plus => 1,
            TokenValue::Minus => 1,
            TokenValue::Mul => 2,
            TokenValue::Div => 2,
            TokenValue::Pow => 3,
            TokenValue::Identifier(_) => 4,
            TokenValue::Number(_) => 4, // Numbers are evaluated right away because they don't require operands
            TokenValue::Function(_) => 4,
            TokenValue::ParenOpen => 5,
        }
    }


    pub fn max_priority() -> Priority {
        Self::ParenOpen.base_priority()
    }

}


pub struct Token<'a> {
    pub value: TokenValue<'a>,
    pub source: Rc<SourceToken<'a>>,
}


fn lex<'a>(source: &'a str) -> impl Iterator<Item = SourceToken<'a>> {

    let source = source.trim();

    TOKEN_REGEX.find_iter(source)
        .map(|mat| 
            match mat.as_str() {
                s => SourceToken {
                    string: s,
                    column: mat.start() + 1
                }
            }
        ).into_iter()
}


pub fn tokenize<'a>(source: &'a str) -> UnparsedTree<'a> {

    let raw_tokens = lex(source);

    let mut tokens = UnparsedTree::new(source);

    let mut positional_priority: Priority = 0;

    for token in raw_tokens {
        match token.string {

            "+" => tokens.push_token(
                Token {
                    value: TokenValue::Plus,
                    source: Rc::new(token)
                },
                positional_priority
            ),
            
            "-" => tokens.push_token(
                Token {
                    value: TokenValue::Minus,
                    source: Rc::new(token)
                },
                positional_priority
            ),

            "*" => tokens.push_token(
                Token {
                    value: TokenValue::Mul,
                    source: Rc::new(token)
                },
                positional_priority
            ),

            "/" => tokens.push_token(
                Token {
                    value: TokenValue::Div,
                    source: Rc::new(token)
                },
                positional_priority
            ),

            "^" => tokens.push_token(
                Token {
                    value: TokenValue::Pow,
                    source: Rc::new(token)
                },
                positional_priority
            ),

            "(" => {
                tokens.push_token(
                    Token {
                        value: TokenValue::ParenOpen,
                        source: Rc::new(token)
                    },
                    positional_priority
                );
                positional_priority += TokenValue::max_priority();
            },

            ")" => {
                positional_priority -= TokenValue::max_priority();
                tokens.push_token(
                    Token {
                        value: TokenValue::ParenClose,
                        source: Rc::new(token)
                    },
                    positional_priority
                );
            },

            string => {

                if let Ok(n) = string.parse::<f64>() {
                    tokens.push_token(
                        Token {
                            value: TokenValue::Number(n),
                            source: Rc::new(token)
                        },
                        positional_priority
                    );
                } else if let Some(function) = Functions::from_name(string) {
                    tokens.push_token(
                        Token {
                            value: TokenValue::Function(function),
                            source: Rc::new(token)
                        }, positional_priority
                    );
                } else if TOKEN_REGEX.is_match(string) {
                    tokens.push_token(
                        Token {
                            value: TokenValue::Identifier(string),
                            source: Rc::new(token),
                        },
                        positional_priority
                    );
                } else {
                    errors::invalid_token(&token, source, "String is not a valid token.")
                }
            }
        }
    }

    tokens
}


pub fn is_variable<'a>(var: &'a str) -> bool {
    VARIABLE_REGEX.is_match(var)
}

