use std::{cell::RefCell, rc::Rc};

use crate::{
    expr::Expr,
    function::{Function, FunctionTable},
};

impl Expr {
    fn find_var(&self, var: String, args: &Vec<Expr>) -> bool {
        for i in 0..args.len() {
            if let Expr::Var(name) = args[i].clone()
                && name == var
            {
                return true;
            }
        }

        false
    }

    fn derivative(
        &self,
        dx: &String,
        args: &Vec<Expr>,
        function_table: Rc<RefCell<FunctionTable>>,
    ) -> Result<Expr, anyhow::Error> {
        if let Expr::Const(_) = self {
            Ok(Expr::Const(0.0))
        } else if let Expr::Var(x) = self {
            if self.find_var(*x, args) {
                if *dx == *x {
                    Ok(Expr::Var(*dx))
                } else {
                    Ok(Expr::Const(1.0))
                }
            } else {
                Err(anyhow::Error::msg(format!("unknown var: {}", x)))
            }
        } else if let Expr::Add(x, y) = self {
            let l = x.as_ref().derivative(dx, args, function_table.clone())?;
            let r = y.as_ref().derivative(dx, args, function_table.clone())?;
            Ok(Expr::Add(Box::<Expr>::new(l), Box::<Expr>::new(r)))
        } else if let Expr::Sub(x, y) = self {
            let l = x.as_ref().derivative(dx, args, function_table.clone())?;
            let r = y.as_ref().derivative(dx, args, function_table.clone())?;
            Ok(Expr::Sub(Box::<Expr>::new(l), Box::<Expr>::new(r)))
        } else if let Expr::Mul(x, y) = self {
            let l = x.as_ref().derivative(dx, args, function_table.clone())?;
            let r = y.as_ref().derivative(dx, args, function_table.clone())?;
            Ok(Expr::Mul(Box::<Expr>::new(l), Box::<Expr>::new(r)))
        } else if let Expr::Div(x, y) = self {
            let l = x.as_ref().derivative(dx, args, function_table.clone())?;
            let r = y.as_ref().derivative(dx, args, function_table.clone())?;
            Ok(Expr::Div(Box::<Expr>::new(l), Box::<Expr>::new(r)))
        } else if let Expr::Power(x, y) = self {
            let base = x.as_ref().derivative(dx, args, function_table.clone())?;
            if let Expr::Var(_) = base {
                let p = y.as_ref().derivative(dx, args, function_table.clone())?;
                if let Expr::Var(_) = p {
                    todo!()
                } else {
                    let mut k=Expr::Sub(
                            Box::<Expr>::new(p),
                            Box::<Expr>::new(Expr::Const(1.0)),
                        );

                    let fk=Function::new_with_expr(k);
                    fk.symble = Expr::Func("".to_string(), args.clone());
                    let tvec=(0..args.len()).collect();
                    if fk.caculate(&tvec, function_table.clone())?==0.0{

                    }

                    let box_p = Box::<Expr>::new(Expr::Power(
                        Box::<Expr>::new(base),
                        Box::<Expr>::new(k),
                    ));
                    return Ok(Expr::Mul(Box::<Expr>::new(p), box_p));
                }
            } else {
                Ok(Expr::Const(1.0))
            }
        } else if let Expr::Log(x, y) = self {
            if let Expr::Var(_) = x.as_ref() {
                todo!()
            } else {
                if let Expr::Var(p) = y.as_ref() {
                    let inner=y.as_ref().derivative(dx, args, function_table.clone())?;

                    if let Expr::Var(_)=inner{

                    }
                    todo!()
                }else{
                    Ok(Expr::Const(1.0))
                }
            }
        } else if let Expr::Trifuncs(ref name, pvar) = self {
            let mut var_func = Function::new_with_expr(*pvar);
            var_func.symble = Expr::Func("".to_string(), orig_args.clone());

            Ok(self.caculate_trifuncs(name, var_func.caculate(arg, function_table)?)?)
        } else if let Expr::Func(name, args) = self {
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
