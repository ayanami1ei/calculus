use std::{cell::RefCell, rc::Rc};

use crate::{
    expr::Expr,
    function::{Function, FunctionTable},
};

impl Function {
    fn find_var(&self, var: String, args: &Vec<f64>) -> Result<f64, anyhow::Error> {
        if let Expr::Func(_, ref orig_args) = self.symble {
            if args.len() != orig_args.len() {
                return Err(anyhow::Error::msg(format!(
                    "argument length mismatch in find_var, args.len()={} orig_args.len()={}",
                    args.len(),
                    orig_args.len()
                )));
            }
            for i in 0..orig_args.len() {
                if let Expr::Var(name) = orig_args[i].clone()
                    && name == var
                {
                    return Ok(args[i]);
                }
            }
        }

        Err(anyhow::Error::msg(format!("unknown variable: {}", var)))
    }

    fn caculate_trifuncs(&self, name: &String, var: f64) -> Result<f64, anyhow::Error> {
        if name == "sin" {
            return Ok(var.sin());
        } else if name == "cos" {
            return Ok(var.cos());
        } else if name == "tan" {
            return Ok(var.tan());
        } else if name == "arcsin" {
            return Ok(var.asin());
        } else if name == "arccos" {
            return Ok(var.acos());
        } else if name == "arctan" {
            return Ok(var.atan());
        } else if name == "csc" {
            return Ok(1.0/var.sin());
        } else if name == "sec" {
            return Ok(1.0/var.cos());
        } else if name == "sot" {
            return Ok(1.0/var.tan());
        }

        Err(anyhow::Error::msg(format!(
            "unknown trigonometric function: {}",
            name
        )))
    }

    pub fn caculate(
        &self,
        arg: &Vec<f64>,
        function_table: Rc<RefCell<FunctionTable>>,
    ) -> Result<f64, anyhow::Error> {
        let pos = self.body.clone();
        let orig_args;
        if let Expr::Func(_, ref _orig_args) = self.symble {
            orig_args = _orig_args.clone();
        } else {
            return Err(anyhow::Error::msg("unknown function"));
        }

        if let Expr::Const(x) = pos {
            #[cfg(debug_assertions)]
            {
                println!("Expr::Const({})", x);
            }
            Ok(x)
        } else if let Expr::Var(x) = pos {
            #[cfg(debug_assertions)]
            {
                println!("Expr::Var({})", x);
            }
            match self.find_var(x, arg) {
                Ok(v) => Ok(v),
                Err(e) => Err(anyhow::Error::msg(format!("unknown var. {}", e))),
            }
        } else if let Expr::Add(x, y) = pos {
            let mut l = Function::new_with_expr(*x);
            let mut r = Function::new_with_expr(*y);

            l.symble = Expr::Func("".to_string(), orig_args.clone());
            r.symble = Expr::Func("".to_string(), orig_args.clone());

            Ok(l.caculate(arg, function_table.clone())?
                + r.caculate(arg, function_table.clone())?)
        } else if let Expr::Sub(x, y) = pos {
            let mut l = Function::new_with_expr(*x);
            let mut r = Function::new_with_expr(*y);

            l.symble = Expr::Func("".to_string(), orig_args.clone());
            r.symble = Expr::Func("".to_string(), orig_args.clone());

            Ok(l.caculate(arg, function_table.clone())?
                - r.caculate(arg, function_table.clone())?)
        } else if let Expr::Mul(x, y) = pos {
            let mut l = Function::new_with_expr(*x);
            let mut r = Function::new_with_expr(*y);

            l.symble = Expr::Func("".to_string(), orig_args.clone());
            r.symble = Expr::Func("".to_string(), orig_args.clone());

            Ok(l.caculate(arg, function_table.clone())?
                * r.caculate(arg, function_table.clone())?)
        } else if let Expr::Div(x, y) = pos {
            let mut l = Function::new_with_expr(*x);
            let mut r = Function::new_with_expr(*y);

            l.symble = Expr::Func("".to_string(), orig_args.clone());
            r.symble = Expr::Func("".to_string(), orig_args.clone());

            Ok(l.caculate(arg, function_table.clone())?
                / r.caculate(arg, function_table.clone())?)
        } else if let Expr::Power(x, y) = pos {
            let mut l = Function::new_with_expr(*x);
            let mut r = Function::new_with_expr(*y);

            l.symble = Expr::Func("".to_string(), orig_args.clone());
            r.symble = Expr::Func("".to_string(), orig_args.clone());

            Ok(l.caculate(arg, function_table.clone())?
                .powf(r.caculate(arg, function_table.clone())?))
        } else if let Expr::Log(x, y) = pos {
            let mut l = Function::new_with_expr(*x);
            let mut r = Function::new_with_expr(*y);

            l.symble = Expr::Func("".to_string(), orig_args.clone());
            r.symble = Expr::Func("".to_string(), orig_args.clone());

            #[cfg(debug_assertions)]
            {
                println!(
                    "log({},{})",
                    r.caculate(arg, function_table.clone())?,
                    l.caculate(arg, function_table.clone())?
                );
            }

            Ok(r.caculate(arg, function_table.clone())?
                .log(l.caculate(arg, function_table.clone())?))
        } else if let Expr::Trifuncs(ref name, pvar) = pos {
            let mut var_func=Function::new_with_expr(*pvar);
            var_func.symble = Expr::Func("".to_string(), orig_args.clone());

            Ok(self.caculate_trifuncs(name, var_func.caculate(arg, function_table)?)?)
        } else if let Expr::Func(name, args) = pos {
            if let Some(func) = function_table.borrow().find(&name, args.len()) {
                let mut argc = Vec::<f64>::new();
                for i in args.clone() {
                    let tfunc = Function::new_with_expr(i);
                    argc.push(tfunc.caculate(arg, function_table.clone())?)
                }

                #[cfg(debug_assertions)]
                {
                    println!("Expr::Func({},{})", name, args.len());
                }

                match func.caculate(&argc, function_table.clone()) {
                    Ok(v) => return Ok(v),
                    Err(e) => return Err(anyhow::Error::msg(format!("unknown function. {}", e))),
                }
            }
            Err(anyhow::Error::msg("unknown caculation"))
        } else {
            Err(anyhow::Error::msg("unknown caculation"))
        }
    }
}
