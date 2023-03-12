use std::fmt;

#[derive(PartialEq)]
pub enum LiteralValue {
    Nil,
    Bool(bool),
    String(String),
    Number(f64),
}

impl LiteralValue {
    /*
    Most dynamically typed languages aren’t that ascetic.
    Instead, they take the universe of values of all types
    and partition them into two sets, one of which they
    define to be “true”, or “truthful”, or “truthy”, and
    the rest which are “false” or “falsey”.
    This partitioning is somewhat arbitrary and gets weird
    in a few languages.

    Here we follow Ruby’s simple rule: false and nil are falsey,
    and everything else is truthy.
    */
    pub fn is_truthy(&self) -> bool {
        match self {
            LiteralValue::Nil => false,
            LiteralValue::Bool(bool) => *bool,
            _ => true,
        }
    }
}

impl fmt::Display for LiteralValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LiteralValue::Nil => f.write_str("Nil"),
            LiteralValue::Bool(bool) => write!(f, "{}", bool),
            LiteralValue::String(string) => write!(f, "{}", string),
            LiteralValue::Number(number) => write!(f, "{}", number),
        }
    }
}
