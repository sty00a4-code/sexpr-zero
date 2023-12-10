use std::{fmt::Display, num::{ParseIntError, ParseFloatError}};

#[cfg(test)]
mod tests;

/// The representation of an s-expression
#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    List(Vec<Self>),
    Symbol(String),
    String(String),
    Int(i64),
    Float(f64),
}

impl Default for Expression {
    fn default() -> Self {
        Self::List(Vec::default())
    }
}
impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::List(exprs) => write!(f, "({})", exprs.iter().map(|expr| expr.to_string()).collect::<Vec<String>>().join(" ")),
            Expression::Symbol(symbol) => write!(f, "{symbol}"),
            Expression::String(string) => write!(f, "{string:?}"),
            Expression::Int(int) => write!(f, "{int:?}"),
            Expression::Float(float) => write!(f, "{float:?}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    UnclosedString,
    ExpectedClosingParan,
    NoMatchingOpenParan,
    ParseIntError(ParseIntError),
    ParseFloatError(ParseFloatError),
}
impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::UnclosedString => write!(f, "unclosed string"),
            ParseError::ExpectedClosingParan => write!(f, "expected closing ')'"),
            ParseError::NoMatchingOpenParan => write!(f, "')' has no matching '('"),
            ParseError::ParseIntError(err) => write!(f, "error while parsing int, {err}"),
            ParseError::ParseFloatError(err) => write!(f, "error while parsing float, {err}"),
        }
    }
}
impl TryFrom<String> for Expression {
    type Error = ParseError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut chars = value.chars().peekable();
        let mut stack = vec![vec![]];
        while let Some(mut c) = chars.next() {
            if c.is_ascii_whitespace() {
                while let Some(wc) = chars.peek() {
                    if !wc.is_ascii_whitespace() {
                        c = chars.next().unwrap();
                        break;
                    }
                    chars.next();
                }
            }
            match c {
                '(' => {
                    stack.push(vec![])
                }
                ')' => if stack.len() > 1 {
                    let list = stack.pop().unwrap();
                    stack.last_mut().unwrap().push(Self::List(list))
                } else {
                    return Err(ParseError::NoMatchingOpenParan)
                }
                '"' => {
                    let mut string = String::new();
                    let mut closed = false;
                    for c in chars.by_ref() {
                        if c == '"' {
                            closed = true;
                            break;
                        }
                        string.push(c);
                    }
                    if !closed {
                        return Err(ParseError::UnclosedString)
                    }
                    stack.last_mut().unwrap().push(Self::String(string));
                }
                c if c.is_ascii_digit() => {
                    let mut number = String::from(c);
                    while let Some(c) = chars.peek() {
                        if !c.is_ascii_digit() {
                            break;
                        }
                        number.push(chars.next().unwrap());
                    }
                    if chars.peek() == Some(&'.') {
                        number.push(chars.next().unwrap());
                        while let Some(c) = chars.peek() {
                            if !c.is_ascii_digit() {
                                break;
                            }
                            number.push(chars.next().unwrap());
                        }
                        stack.last_mut().unwrap().push(number.parse().map(Self::Float).map_err(ParseError::ParseFloatError)?);
                    } else {
                        stack.last_mut().unwrap().push(Self::Int(number.parse().map_err(ParseError::ParseIntError)?))
                    }
                }
                c => {
                    let mut symbol = String::from(c);
                    for c in chars.by_ref() {
                        if c.is_ascii_whitespace() {
                            break;
                        }
                        symbol.push(c);
                    }
                    stack.last_mut().unwrap().push(Self::Symbol(symbol))
                }
            }
        }
        let mut exprs = stack.pop().unwrap();
        if exprs.len() == 1 {
            Ok(exprs.remove(0))
        } else {
            Ok(Self::List(exprs))
        }
    }
}