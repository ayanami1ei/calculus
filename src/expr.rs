use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Const(f64),
    Var(String),
    Func(String, Vec<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Power(Box<Expr>, Box<Expr>),
    Log(Box<Expr>, Box<Expr>),
    Trifuncs(String, Box<Expr>),
    Equal(Box<Expr>, Box<Expr>),
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.fmt_with_prec(f, 0)
    }
}

impl Expr {
    /// 返回当前表达式的运算优先级（数字越大优先级越高）。
    fn precedence(&self) -> u8 {
        match self {
            Expr::Add(_, _) | Expr::Sub(_, _) => 1,
            Expr::Mul(_, _) | Expr::Div(_, _) => 2,
            Expr::Power(_, _) => 3,
            Expr::Equal(_, _) => 0,
            _ => 4,
        }
    }

    /// 按运算优先级输出表达式，在需要时自动加括号。
    fn fmt_with_prec(&self, f: &mut Formatter<'_>, parent_prec: u8) -> std::fmt::Result {
        let my_prec = self.precedence();
        let need_paren = my_prec < parent_prec;

        if need_paren {
            write!(f, "(")?;
        }

        match self {
            Expr::Const(x) => {
                // 将与自然常数 e 足够接近的常数以符号 e 显示
                if (*x - std::f64::consts::E).abs() < 1e-12 {
                    write!(f, "e")?
                } else {
                    write!(f, "{}", x)?
                }
            }
            Expr::Var(x) => write!(f, "{}", x)?,
            Expr::Add(x, y) => {
                // 左结合：右侧在同一优先级上提升 parent_prec，避免 a-(b-c) 这类歧义
                x.fmt_with_prec(f, my_prec)?;
                write!(f, "+")?;
                y.fmt_with_prec(f, my_prec + 1)?;
            }
            Expr::Sub(x, y) => {
                x.fmt_with_prec(f, my_prec)?;
                write!(f, "-")?;
                y.fmt_with_prec(f, my_prec + 1)?;
            }
            Expr::Mul(x, y) => {
                x.fmt_with_prec(f, my_prec)?;
                write!(f, "*")?;
                // 右侧用更高的 parent_prec，保证 a*(b/c)、a*(b+c) 等形式有括号
                y.fmt_with_prec(f, my_prec + 1)?;
            }
            Expr::Div(x, y) => {
                x.fmt_with_prec(f, my_prec)?;
                write!(f, "/")?;
                // 右侧用更高的 parent_prec，强制 a/(b*c)、a/(b/c) 加括号
                y.fmt_with_prec(f, my_prec + 1)?;
            }
            Expr::Power(x, y) => {
                x.fmt_with_prec(f, my_prec)?;
                write!(f, "^")?;
                // 指数部分整体加括号更安全
                y.fmt_with_prec(f, my_prec + 1)?;
            }
            Expr::Log(x, y) => {
                write!(f, "log(")?;
                x.fmt_with_prec(f, 0)?;
                write!(f, ",")?;
                y.fmt_with_prec(f, 0)?;
                write!(f, ")")?;
            }
            Expr::Trifuncs(name, pvar) => {
                write!(f, "{}(", name)?;
                pvar.fmt_with_prec(f, 0)?;
                write!(f, ")")?;
            }
            Expr::Func(name, args) => {
                write!(f, "{}(", name)?;
                for (i, arg) in args.iter().enumerate() {
                    arg.fmt_with_prec(f, 0)?;
                    if i + 1 != args.len() {
                        write!(f, ",")?;
                    }
                }
                write!(f, ")")?;
            }
            Expr::Equal(x, y) => {
                x.fmt_with_prec(f, my_prec)?;
                write!(f, "=")?;
                y.fmt_with_prec(f, my_prec)?;
            }
        }

        if need_paren {
            write!(f, ")")?;
        }

        Ok(())
    }

    /// 递归地简化表达式树，去掉诸如 +0、*1、*0 等冗余项。
    pub fn simplify(&self) -> Expr {
        match self {
            Expr::Const(c) => Expr::Const(*c),
            Expr::Var(v) => Expr::Var(v.clone()),
            Expr::Func(name, args) => {
                let new_args: Vec<Expr> = args.iter().map(|e| e.simplify()).collect();
                Expr::Func(name.clone(), new_args)
            }
            Expr::Add(x, y) => {
                let l = x.simplify();
                let r = y.simplify();
                match (&l, &r) {
                    (Expr::Const(a), Expr::Const(b)) => Expr::Const(a + b),
                    (Expr::Const(a), _) if *a == 0.0 => r,
                    (_, Expr::Const(b)) if *b == 0.0 => l,
                    _ => Expr::Add(Box::new(l), Box::new(r)),
                }
            }
            Expr::Sub(x, y) => {
                let l = x.simplify();
                let r = y.simplify();
                match (&l, &r) {
                    (Expr::Const(a), Expr::Const(b)) => Expr::Const(a - b),
                    (_, Expr::Const(b)) if *b == 0.0 => l,
                    _ => Expr::Sub(Box::new(l), Box::new(r)),
                }
            }
            Expr::Mul(x, y) => {
                let l = x.simplify();
                let r = y.simplify();
                match (&l, &r) {
                    (Expr::Const(a), Expr::Const(b)) => Expr::Const(a * b),
                    (Expr::Const(a), _) if *a == 0.0 => Expr::Const(0.0),
                    (_, Expr::Const(b)) if *b == 0.0 => Expr::Const(0.0),
                    (Expr::Const(a), _) if *a == 1.0 => r,
                    (_, Expr::Const(b)) if *b == 1.0 => l,
                    _ => Expr::Mul(Box::new(l), Box::new(r)),
                }
            }
            Expr::Div(x, y) => {
                let l = x.simplify();
                let r = y.simplify();
                match (&l, &r) {
                    (Expr::Const(a), Expr::Const(b)) if *b != 0.0 => Expr::Const(a / b),
                    (Expr::Const(a), _) if *a == 0.0 => Expr::Const(0.0),
                    (_, Expr::Const(b)) if *b == 1.0 => l,
                    _ => Expr::Div(Box::new(l), Box::new(r)),
                }
            }
            Expr::Power(x, y) => {
                let l = x.simplify();
                let r = y.simplify();
                match (&l, &r) {
                    (Expr::Const(a), Expr::Const(b)) => Expr::Const(a.powf(*b)),
                    (_, Expr::Const(b)) if *b == 0.0 => Expr::Const(1.0),
                    (_, Expr::Const(b)) if *b == 1.0 => l,
                    (Expr::Const(a), _) if *a == 0.0 => Expr::Const(0.0),
                    (Expr::Const(a), _) if *a == 1.0 => Expr::Const(1.0),
                    _ => Expr::Power(Box::new(l), Box::new(r)),
                }
            }
            Expr::Log(x, y) => {
                let l = x.simplify();
                let r = y.simplify();
                Expr::Log(Box::new(l), Box::new(r))
            }
            Expr::Trifuncs(name, pvar) => {
                let inner = pvar.simplify();
                Expr::Trifuncs(name.clone(), Box::new(inner))
            }
            Expr::Equal(x, y) => {
                let l = x.simplify();
                let r = y.simplify();
                Expr::Equal(Box::new(l), Box::new(r))
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Identifier(String),
    Const(f64),
    Operator(char),
    Log(),
    Trifuncs(String),
}

impl Token {
    pub fn as_identifier(&self) -> Result<String, anyhow::Error> {
        if let Token::Identifier(s) = self {
            Ok(s.to_string())
        } else {
            Err(anyhow::Error::msg(
                "please use Token.Identifier to call as_identifier",
            ))
        }
    }

    pub fn as_const(&self) -> Result<f64, anyhow::Error> {
        if let Token::Const(s) = self {
            Ok(*s)
        } else {
            Err(anyhow::Error::msg(
                "please use Token.Const to call as_const",
            ))
        }
    }

    pub fn as_operator(&self) -> Result<char, anyhow::Error> {
        if let Token::Operator(s) = self {
            Ok(*s)
        } else {
            Err(anyhow::Error::msg(
                "please use Token.Operator to call as_operator",
            ))
        }
    }

    pub fn get_type(&self) -> Result<String, anyhow::Error> {
        if let Token::Identifier(_) = self {
            Ok("identifier".to_string())
        } else if let Token::Const(_) = self {
            Ok("const".to_string())
        } else if let Token::Operator(_) = self {
            Ok("operator".to_string())
        } else if let Token::Log() = self {
            Ok("log".to_string())
        } else {
            Err(anyhow::Error::msg("unkonwn type"))
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Token::Identifier(s) = self {
            write!(f, "Identifier: {}", s)
        } else if let Token::Const(s) = self {
            write!(f, "Const: {}", s)
        } else if let Token::Operator(s) = self {
            write!(f, "Operator: {}", s)
        } else if let Token::Log() = self {
            write!(f, "Log")
        } else if let Token::Trifuncs(name) = self {
            write!(f, "Trifuncs: {}", name)
        } else {
            write!(f, "unknown token")
        }
    }
}
