use super::Monomial;
use std::{ops, str::FromStr};

impl Monomial {
    fn is_zero(&self) -> bool {
        return self.variable == "0" && self.coefficient == 0.0;
    }

    // d/dx (a x^n) = (a*n) x^(n-1); constant turns into 0.
    pub fn derivative(&self) -> Monomial {
        if self.exponent.abs() < f32::EPSILON {
            return Monomial {
                variable: self.variable.clone(),
                coefficient: 0.0,
                exponent: 0.0,
            };
        }

        Monomial {
            variable: self.variable.clone(),
            coefficient: self.coefficient * self.exponent,
            exponent: self.exponent - 1.0,
        }
    }

    // ∫ a x^n dx = a/(n+1) x^(n+1); caller can add constant separately.
    pub fn integral(&self) -> Result<Monomial, String> {
        if (self.exponent + 1.0).abs() < f32::EPSILON {
            return Err("integral has ln term when exponent = -1".to_string());
        }

        Ok(Monomial {
            variable: self.variable.clone(),
            coefficient: self.coefficient / (self.exponent + 1.0),
            exponent: self.exponent + 1.0,
        })
    }

    fn split_poly(&self, s: &str) -> Vec<String> {
        let mut res = Vec::<String>::new();

        let mut token = String::new();
        let mut index = 0;
        for (i, ch) in s.chars().enumerate() {
            if ch.is_ascii_digit() {
                token.push(ch);
            } else if ch == '-' && i == 0 {
                token.push(ch);
            } else {
                index = i;
                break;
            }
        }
        res.push(token);

        token = String::new();
        for (i, ch) in s.chars().enumerate() {
            if i < index {
                continue;
            }

            if ch.is_ascii_alphabetic() {
                token.push(ch);
            } else {
                index=i;
                break;
            }
        }
        res.push(token);

        token = String::new();
        for (i, ch) in s.chars().enumerate() {
            if i <= index {
                continue;
            }

            if ch.is_ascii_digit() {
                token.push(ch);
            } else if ch == '-' && i == index {
                token.push(ch);
            } else {
                break;
            }
        }
        res.push(token);

        res
    }
}

impl PartialEq for Monomial {
    fn eq(&self, other: &Self) -> bool {
        self.variable == other.variable
            && self.coefficient == other.coefficient
            && self.exponent == other.exponent
    }
}

impl ops::Add for Monomial {
    type Output = Result<Monomial, String>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.variable != rhs.variable {
            return Err("variable must match".to_string());
        }

        if (self.exponent - rhs.exponent).abs() > f32::EPSILON {
            return Err("exponents must match".to_string());
        }

        Ok(Monomial {
            variable: self.variable,
            coefficient: self.coefficient + rhs.coefficient,
            exponent: self.exponent,
        })
    }
}

impl ops::Sub for Monomial {
    type Output = Result<Monomial, String>;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.variable != rhs.variable {
            return Err("variable must match".to_string());
        }

        if (self.exponent - rhs.exponent).abs() > f32::EPSILON {
            return Err("exponents must match".to_string());
        }

        Ok(Monomial {
            variable: self.variable,
            coefficient: self.coefficient - rhs.coefficient,
            exponent: self.exponent,
        })
    }
}

impl ops::Mul<f32> for Monomial {
    type Output = Result<Monomial, String>;

    fn mul(self, rhs: f32) -> Self::Output {
        Ok(Monomial {
            variable: self.variable,
            coefficient: self.coefficient * rhs,
            exponent: self.exponent,
        })
    }
}

impl ops::Div<f32> for Monomial {
    type Output = Result<Monomial, String>;

    fn div(self, rhs: f32) -> Self::Output {
        if rhs == 0.0 {
            return Err("can't divide zero".to_string());
        }

        Ok(Monomial {
            variable: self.variable,
            coefficient: self.coefficient / rhs,
            exponent: self.exponent,
        })
    }
}

impl std::fmt::Display for Monomial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}^{}", self.coefficient, self.variable, self.exponent)
    }
}

impl FromStr for Monomial {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let binding = s.to_string();
        let mut res:Monomial=Monomial {
            variable: "".to_string(),
            coefficient: 0.0,
            exponent: 0.0,
        };
        let vec=res.split_poly(&binding);

        match vec[0].parse::<f32>() {
            Ok(v) => {
                res.coefficient = v;
            }
            Err(_) => return Err("error in parsing coefficient".to_string()),
        }

        match vec[2].parse::<f32>() {
            Ok(v) => {
                res.exponent = v;
            }
            Err(_) => return Err("error in parsing exponent".to_string()),
        }

        res.variable=vec[1].clone();

        Ok(res)
    }
}