use std::fmt::{Display};

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
        if let Expr::Const(x) = self {
            write!(f, "{}", x)
        } else if let Expr::Var(x) = self {
            write!(f, "{}", x)
        } else if let Expr::Add(x, y) = self {
            write!(f, "{}+{}", x.as_ref(),y.as_ref())
        } else if let Expr::Sub(x, y) = self {
            write!(f, "{}-{}", x.as_ref(),y.as_ref())
        } else if let Expr::Mul(x, y) = self {
            write!(f, "{}*{}", x.as_ref(),y.as_ref())
        } else if let Expr::Div(x, y) = self {
            write!(f, "{}/{}", x.as_ref(),y.as_ref())
        } else if let Expr::Power(x, y) = self {
            write!(f, "{}^{}", x.as_ref(),y.as_ref())
        } else if let Expr::Log(x, y) = self {
            write!(f, "log({},{})", x.as_ref(),y.as_ref())
        } else if let Expr::Trifuncs(name, pvar) = self {
            write!(f, "{}({})", name,pvar.as_ref())
        } else if let Expr::Func(name, args) = self {
            write!(f,"{}(",name)?;
            for i in 0..args.len(){
                write!(f, "{}",args[i])?;
                if i!=args.len()-1{
                    write!(f,",")?;
                }
            }
            write!(f, ")")
        } else {
            write!(f, "function cannot diaplay")
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
