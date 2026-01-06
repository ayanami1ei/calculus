use std::{cell::RefCell, rc::Rc};

use crate::{
    expr::{Expr, Token},
    function::{Function, FunctionTable},
};

impl Function {
    fn try_call(
        &mut self,
        name: &str,
        function_table: Rc<RefCell<FunctionTable>>,
    ) -> Result<Expr, anyhow::Error> {
        let mut count = 0;
        let mut args = Vec::<Expr>::new();

        while !self.is(Token::Operator(')'))? {
            args.push(self.parse_add_or_sub(function_table.clone())?);
            count += 1;
        }

        let binding = function_table.borrow();
        let res_func = binding.find(name, count);

        if res_func == None {
            return Err(anyhow::Error::msg("unknown function"));
        }

        Ok(Expr::Func(name.to_string(), args))
    }

    pub(crate) fn parse_add_or_sub(
        &mut self,
        function_table: Rc<RefCell<FunctionTable>>,
    ) -> Result<Expr, anyhow::Error> {
        let mut left = self.parse_mul_or_div(function_table.clone())?;

        loop {
            if self.is(Token::Operator('+'))? {
                #[cfg(debug_assertions)]
                {
                    println!("+");
                }
                let right = self.parse_mul_or_div(function_table.clone())?;
                left = Expr::Add(Box::new(left), Box::new(right));
            } else if self.is(Token::Operator('-'))? {
                #[cfg(debug_assertions)]
                {
                    println!("-");
                }
                let right = self.parse_mul_or_div(function_table.clone())?;
                left = Expr::Sub(Box::new(left), Box::new(right));
            } else {
                break;
            }
        }

        Ok(left)
    }

    fn parse_mul_or_div(
        &mut self,
        function_table: Rc<RefCell<FunctionTable>>,
    ) -> Result<Expr, anyhow::Error> {
        let mut left = self.parse_power(function_table.clone())?;

        loop {
            if self.is(Token::Operator('*'))? {
                #[cfg(debug_assertions)]
                {
                    println!("*");
                }
                let right = self.parse_power(function_table.clone())?;
                left = Expr::Mul(Box::<Expr>::new(left), Box::<Expr>::new(right));
            } else if self.is(Token::Operator('/'))? {
                #[cfg(debug_assertions)]
                {
                    println!("/");
                }
                let right = self.parse_power(function_table.clone())?;
                left = Expr::Div(Box::<Expr>::new(left), Box::<Expr>::new(right));
            } else {
                #[cfg(debug_assertions)]
                {
                    if self.i < self.tokens.len() {
                        println!("{}", self.tokens[self.i]);
                    }
                }
                break;
            }
        }

        Ok(left)
    }

    fn parse_power(
        &mut self,
        function_table: Rc<RefCell<FunctionTable>>,
    ) -> Result<Expr, anyhow::Error> {
        let mut left = self.parse_primary(function_table.clone())?;

        if self.is(Token::Operator('^'))? {
            let right = self.parse_primary(function_table.clone())?;
            left = Expr::Power(Box::<Expr>::new(left), Box::<Expr>::new(right));
        }

        Ok(left)
    }

    fn parse_primary(
        &mut self,
        function_table: Rc<RefCell<FunctionTable>>,
    ) -> Result<Expr, anyhow::Error> {
        let peek = self.tokens[self.i].clone();
        let mut res;

        #[cfg(debug_assertions)]
        {
            println!("peek={}", peek);
        }

        if self.is(Token::Operator('('))? {
            res = self.parse_add_or_sub(function_table.clone())?;
            while self.is(Token::Operator(','))? {
                res = self.parse_add_or_sub(function_table.clone())?;
            }

            self.expect(Token::Operator(')'))?;
        } else if let Token::Identifier(_) = peek {
            self.i += 1;

            if self.is(Token::Operator('('))? {
                println!("peek={}",peek);
                res = self.try_call(&peek.as_identifier()?, function_table)?;
            } else {
                res = Expr::Var(peek.as_identifier()?);
            }
        } else if let Token::Const(_) = peek {
            self.i += 1;
            res = Expr::Const(peek.as_const()?);
        } else if let Token::Log() = peek {
            self.i += 1;
            self.expect(Token::Operator('('))?;

            let l = self.parse_add_or_sub(function_table.clone())?;
            let r = self.parse_add_or_sub(function_table.clone())?;

            self.expect(Token::Operator(')'))?;

            res = Expr::Log(Box::<Expr>::new(l), Box::<Expr>::new(r));
        } else if let Token::Trifuncs(ref name) = peek {
            self.i += 1;
            self.expect(Token::Operator('('))?;

            let var = self.parse_add_or_sub(function_table.clone())?;

            self.expect(Token::Operator(')'))?;

            res = Expr::Trifuncs(name.clone(), Box::<Expr>::new(var));
        } else {
            return Err(anyhow::Error::msg("unkonwn type"));
        }

        Ok(res)
    }
}
