use std::collections::HashMap;

use crate::expr::{Expr, Token};
pub mod caculate;
pub mod derivative;
pub mod implement;
pub mod parse;

#[derive(Clone, PartialEq)]
pub struct Function {
    pub(crate) symble: Expr,
    body: Expr,

    tokens: Vec<Token>,
    i: usize,
}

#[derive(Clone)]
pub struct FunctionTable {
    map: HashMap<String, Function>,
}
