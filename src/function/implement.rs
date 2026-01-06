use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    expr::{Expr, Token},
    function::{Function, FunctionTable},
};

impl Function {
    pub(super) fn expect(&mut self, token: Token) -> Result<(), anyhow::Error> {
        if self.i >= self.tokens.len() {
            return Err(anyhow::Error::msg("index over tokens.len()"));
        }

        if self.tokens[self.i] == token {
            self.i += 1;
            return Ok(());
        }

        let mut err = "expect ".to_string();
        err.push_str(&token.to_string());
        err.push_str(" but find ");
        err.push_str(&self.tokens[self.i].to_string());

        Err(anyhow::Error::msg(err))
    }

    pub(super) fn is(&mut self, token: Token) -> Result<bool, anyhow::Error> {
        if self.i >= self.tokens.len() {
            return Ok(false)
        }

        if self.tokens[self.i] == token {
            self.i += 1;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn generate_body(&mut self) -> Result<Expr, anyhow::Error> {
        let res: Expr = Expr::Const(0.0);

        self.expect(Token::Operator('='))?;

        Ok(res)
    }

    fn generate_name(&mut self) -> Result<(), anyhow::Error> {
        self.i += 1;

        self.expect(Token::Operator('('))?;
        let mut args = Vec::<Expr>::new();

        while !self.is(Token::Operator(')'))? {
            args.push(Expr::Var(self.tokens[self.i].as_identifier()?));
            self.i += 1;

            if self.is(Token::Operator(','))? {
                continue;
            } else {
                #[cfg(debug_assertions)]
                {
                    println!(
                        "generate_name: {}",
                        self.tokens[self.i - 1].as_identifier()?
                    );
                }
            }
        }

        #[cfg(debug_assertions)]{
            println!("args.len()={}",args.len());
        }

        if let Expr::Func(name, _) = &self.symble {
            self.symble = Expr::Func(name.clone(), args);
        } else {
            return Err(anyhow::Error::msg("need Expr::Func"));
        }

        Ok(())
    }

    pub(crate) fn new(
        tokens: &Vec<Token>,
        function_table: Rc<RefCell<FunctionTable>>,
    ) -> Result<Function, anyhow::Error> {
        let name = tokens[0].as_identifier()?;
        let args = Vec::<Expr>::new();
        let root = Expr::Const(0.0);

        let mut res = Function {
            symble: Expr::Func(name.clone(), args),
            root,
            body: Expr::Const(0.0),

            tokens: tokens.to_vec(),
            i: 0,
        };

        res.generate_name()?;
        res.root = res.generate_body()?;
        let mut _binding=function_table.clone();
        let mut binding=_binding.borrow_mut();

        match binding.check_duplicate(&res) {
            Ok(true) => {
                // 已存在同名同参且表达式相同的函数：不必再次插入
            }
            Ok(false) => {
                // 没有同名同参的函数，可以安全插入
                binding
                    .insert(name.clone(), res.clone());
            }
            Err(e) => {
                // 存在同名同参但表达式不同，视为冲突
                return Err(e);
            }
        }

        res.body = res.parse_add_or_sub(function_table)?;

        Ok(res)
    }

    pub(super) fn new_with_expr(body: Expr) -> Function {
        Function {
            symble: Expr::Func("".to_string(), Vec::new()),
            root: Expr::Const(0.0),
            body,

            tokens: Vec::new(),
            i: 0,
        }
    }
}

impl FunctionTable {
    pub fn new() -> FunctionTable {
        FunctionTable {
            map: HashMap::new(),
        }
    }

    pub fn insert(&mut self, name: String, func: Function) {
        self.map.insert(name, func);
    }

    pub fn find(&self, name: &str, argc: usize) -> Option<&Function> {
        if let Some(func) = self.map.get(name) {
            if let Expr::Func(_, args) = &func.symble {
                if args.len() == argc {
                    return Some(func);
                }
            }
        }
        None
    }

    pub fn check_duplicate(&self, func: &Function) -> Result<bool, anyhow::Error> {
        if let Expr::Func(name, args) = &func.symble {
            if let Some(existing) = self.map.get(name) {
                if let Expr::Func(_, exist_args) = &existing.symble {
                    if exist_args.len() == args.len() {
                        if existing.root == func.root {
                            return Ok(true);
                        } else {
                            return Err(anyhow::Error::msg(
                                "function with same name and arg count has different expression",
                            ));
                        }
                    }
                }
            }
            Ok(false)
        } else {
            Err(anyhow::Error::msg("expect Expr::Func"))
        }
    }
}

impl std::fmt::Display for Function{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}={}",self.symble,self.body)
    }
}