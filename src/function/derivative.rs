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

    /// 对表达式按变量 `dx` 求导。
    fn derivative(
        &self,
        dx: &String,
        args: &Vec<Expr>,
        function_table: Rc<RefCell<FunctionTable>>,
    ) -> Result<Expr, anyhow::Error> {
        match self {
            Expr::Const(_) => Ok(Expr::Const(0.0)),
            Expr::Var(x) => {
                if self.find_var(x.clone(), args) {
                    if dx == x {
                        Ok(Expr::Const(1.0))
                    } else {
                        Ok(Expr::Const(0.0))
                    }
                } else {
                    Err(anyhow::Error::msg(format!("unknown var: {}", x)))
                }
            }
            Expr::Add(x, y) => {
                let l = x.as_ref().derivative(dx, args, function_table.clone())?;
                let r = y.as_ref().derivative(dx, args, function_table.clone())?;
                Ok(Expr::Add(Box::new(l), Box::new(r)))
            }
            Expr::Sub(x, y) => {
                let l = x.as_ref().derivative(dx, args, function_table.clone())?;
                let r = y.as_ref().derivative(dx, args, function_table.clone())?;
                Ok(Expr::Sub(Box::new(l), Box::new(r)))
            }
            Expr::Mul(x, y) => {
                // (u * v)' = u' * v + u * v'
                let u = x.as_ref();
                let v = y.as_ref();
                let du = u.derivative(dx, args, function_table.clone())?;
                let dv = v.derivative(dx, args, function_table.clone())?;
                let term1 = Expr::Mul(Box::new(du), Box::new(v.clone()));
                let term2 = Expr::Mul(Box::new(u.clone()), Box::new(dv));
                Ok(Expr::Add(Box::new(term1), Box::new(term2)))
            }
            Expr::Div(x, y) => {
                // (u / v)' = (u' * v - u * v') / v^2
                let u = x.as_ref();
                let v = y.as_ref();
                let du = u.derivative(dx, args, function_table.clone())?;
                let dv = v.derivative(dx, args, function_table.clone())?;
                let num_left = Expr::Mul(Box::new(du), Box::new(v.clone()));
                let num_right = Expr::Mul(Box::new(u.clone()), Box::new(dv));
                let numerator = Expr::Sub(Box::new(num_left), Box::new(num_right));
                let denom = Expr::Power(Box::new(v.clone()), Box::new(Expr::Const(2.0)));
                Ok(Expr::Div(Box::new(numerator), Box::new(denom)))
            }
            Expr::Power(x, y) => {
                // 一般情形：f(x) = u^v
                // 若 u、v 都为常数，则导数为 0
                let u = x.as_ref();
                let v = y.as_ref();
                if let (Expr::Const(_), Expr::Const(_)) = (u, v) {
                    return Ok(Expr::Const(0.0));
                }

                let du = u.derivative(dx, args, function_table.clone())?;
                let dv = v.derivative(dx, args, function_table.clone())?;

                match (u, v) {
                    // f(x)^c,  c 为常数: (u^c)' = c * u^{c-1} * u'
                    (_, Expr::Const(c)) => {
                        let pow = Expr::Power(Box::new(u.clone()), Box::new(Expr::Const(c - 1.0)));
                        let coef_mul_pow = Expr::Mul(Box::new(Expr::Const(*c)), Box::new(pow));
                        Ok(Expr::Mul(Box::new(coef_mul_pow), Box::new(du)))
                    }
                    // a^{f(x)}, a 为常数: (a^v)' = a^v * ln(a) * v'
                    (Expr::Const(a), _) => {
                        let pow = Expr::Power(Box::new(Expr::Const(*a)), Box::new(v.clone()));
                        let ln_a = Expr::Log(
                            Box::new(Expr::Const(*a)),
                            Box::new(Expr::Const(std::f64::consts::E)),
                        );
                        let inner = Expr::Mul(Box::new(ln_a), Box::new(dv));
                        Ok(Expr::Mul(Box::new(pow), Box::new(inner)))
                    }
                    // 一般 u(x)^v(x): (u^v)' = u^v * (v' * ln u + v * u'/u)
                    (_, _) => {
                        let pow = Expr::Power(Box::new(u.clone()), Box::new(v.clone()));
                        let ln_u = Expr::Log(
                            Box::new(u.clone()),
                            Box::new(Expr::Const(std::f64::consts::E)),
                        );
                        let term1 = Expr::Mul(Box::new(dv), Box::new(ln_u));
                        let u_div = Expr::Div(Box::new(du), Box::new(u.clone()));
                        let term2 = Expr::Mul(Box::new(v.clone()), Box::new(u_div));
                        let sum = Expr::Add(Box::new(term1), Box::new(term2));
                        Ok(Expr::Mul(Box::new(pow), Box::new(sum)))
                    }
                }
            }
            Expr::Log(x, y) => {
                // log_u(v) = ln(v) / ln(u)
                let u = x.as_ref();
                let v = y.as_ref();

                // 若 u、v 都是常数，视为常数函数，导数为 0
                if let (Expr::Const(_), Expr::Const(_)) = (u, v) {
                    return Ok(Expr::Const(0.0));
                }

                let du = u.derivative(dx, args, function_table.clone())?;
                let dv = v.derivative(dx, args, function_table.clone())?;

                // A = ln v, B = ln u
                // (A/B)' = (A' * B - A * B') / B^2
                let ln_v = Expr::Log(
                    Box::new(v.clone()),
                    Box::new(Expr::Const(std::f64::consts::E)),
                );
                let ln_u = Expr::Log(
                    Box::new(u.clone()),
                    Box::new(Expr::Const(std::f64::consts::E)),
                );

                // A' = v'/v
                let dv_over_v = Expr::Div(Box::new(dv), Box::new(v.clone()));
                // B' = u'/u
                let du_over_u = Expr::Div(Box::new(du), Box::new(u.clone()));

                let term1 = Expr::Mul(Box::new(dv_over_v), Box::new(ln_u.clone()));
                let term2 = Expr::Mul(Box::new(ln_v.clone()), Box::new(du_over_u));
                let numerator = Expr::Sub(Box::new(term1), Box::new(term2));

                let denom = Expr::Power(Box::new(ln_u.clone()), Box::new(Expr::Const(2.0)));
                Ok(Expr::Div(Box::new(numerator), Box::new(denom)))
            }
            Expr::Trifuncs(name, pvar) => {
                let inner = pvar.as_ref();
                let din = inner.derivative(dx, args, function_table.clone())?;

                match name.as_str() {
                    // (sin g)' = cos(g) * g'
                    "sin" => {
                        let cos_g = Expr::Trifuncs("cos".to_string(), Box::new(inner.clone()));
                        Ok(Expr::Mul(Box::new(cos_g), Box::new(din)))
                    }
                    // (cos g)' = -sin(g) * g'
                    "cos" => {
                        let sin_g = Expr::Trifuncs("sin".to_string(), Box::new(inner.clone()));
                        let prod = Expr::Mul(Box::new(sin_g), Box::new(din));
                        Ok(Expr::Mul(Box::new(Expr::Const(-1.0)), Box::new(prod)))
                    }
                    // (tan g)' = sec^2(g) * g'
                    "tan" => {
                        let sec_g = Expr::Trifuncs("sec".to_string(), Box::new(inner.clone()));
                        let sec_sq = Expr::Power(Box::new(sec_g), Box::new(Expr::Const(2.0)));
                        Ok(Expr::Mul(Box::new(sec_sq), Box::new(din)))
                    }
                    // (arcsin g)' = g' / sqrt(1 - g^2)
                    "arcsin" => {
                        let g2 = Expr::Power(Box::new(inner.clone()), Box::new(Expr::Const(2.0)));
                        let one_minus = Expr::Sub(Box::new(Expr::Const(1.0)), Box::new(g2));
                        let sqrt = Expr::Power(Box::new(one_minus), Box::new(Expr::Const(0.5)));
                        Ok(Expr::Div(Box::new(din), Box::new(sqrt)))
                    }
                    // (arccos g)' = -g' / sqrt(1 - g^2)
                    "arccos" => {
                        let g2 = Expr::Power(Box::new(inner.clone()), Box::new(Expr::Const(2.0)));
                        let one_minus = Expr::Sub(Box::new(Expr::Const(1.0)), Box::new(g2));
                        let sqrt = Expr::Power(Box::new(one_minus), Box::new(Expr::Const(0.5)));
                        let frac = Expr::Div(Box::new(din), Box::new(sqrt));
                        Ok(Expr::Mul(Box::new(Expr::Const(-1.0)), Box::new(frac)))
                    }
                    // (arctan g)' = g' / (1 + g^2)
                    "arctan" => {
                        let g2 = Expr::Power(Box::new(inner.clone()), Box::new(Expr::Const(2.0)));
                        let denom = Expr::Add(Box::new(Expr::Const(1.0)), Box::new(g2));
                        Ok(Expr::Div(Box::new(din), Box::new(denom)))
                    }
                    // (csc g)' = -csc(g) * cot(g) * g'
                    "csc" => {
                        let csc_g = Expr::Trifuncs("csc".to_string(), Box::new(inner.clone()));
                        let cot_g = Expr::Trifuncs("sot".to_string(), Box::new(inner.clone()));
                        let prod = Expr::Mul(Box::new(csc_g), Box::new(cot_g));
                        let prod = Expr::Mul(Box::new(prod), Box::new(din));
                        Ok(Expr::Mul(Box::new(Expr::Const(-1.0)), Box::new(prod)))
                    }
                    // (sec g)' = sec(g) * tan(g) * g'
                    "sec" => {
                        let sec_g = Expr::Trifuncs("sec".to_string(), Box::new(inner.clone()));
                        let tan_g = Expr::Trifuncs("tan".to_string(), Box::new(inner.clone()));
                        let prod = Expr::Mul(Box::new(sec_g), Box::new(tan_g));
                        Ok(Expr::Mul(Box::new(prod), Box::new(din)))
                    }
                    // (cot g)' = -csc^2(g) * g'
                    "cot" => {
                        let csc_g = Expr::Trifuncs("csc".to_string(), Box::new(inner.clone()));
                        let csc_sq = Expr::Power(Box::new(csc_g), Box::new(Expr::Const(2.0)));
                        let prod = Expr::Mul(Box::new(csc_sq), Box::new(din));
                        Ok(Expr::Mul(Box::new(Expr::Const(-1.0)), Box::new(prod)))
                    }
                    _ => Err(anyhow::Error::msg(format!(
                        "unknown trigonometric function for derivative: {}",
                        name
                    ))),
                }
            }
            Expr::Func(_, _) | Expr::Equal(_, _) => Err(anyhow::Error::msg(
                "derivative for this expression type is not implemented",
            )),
        }
    }
}

impl Function {
    pub(crate) fn derivative(
        &self,
        dx: &String,
        function_table: Rc<RefCell<FunctionTable>>,
    ) -> Result<Function, anyhow::Error> {
        if let Expr::Func(mut name, args) = self.symble.clone() {
            let body = self
                .body
                .derivative(dx, &args, function_table.clone())?
                .simplify();
            name.push('\'');
            Ok(Function {
                symble: Expr::Func(name, args),
                body: body,
                tokens: Vec::new(),
                i: 0,
            })
        } else {
            Err(anyhow::Error::msg("illegal function"))
        }
    }
}
