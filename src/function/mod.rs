use std::collections::HashMap;

use crate::expr::{Expr, Token};
pub mod implement;
pub mod parse;
pub mod caculate;
//pub mod derivative;

#[derive(Clone, PartialEq)]
pub struct Function {
    symble: Expr,
    root: Expr,
    
    body: Expr,

    tokens: Vec<Token>,
    i: usize,
}

#[derive(Clone)]
pub struct FunctionTable {
    map: HashMap<String, Function>,
}
