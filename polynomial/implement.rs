use super::Polynomial;
use crate::monomial::Monomial;

impl Polynomial {
	pub fn new(monomials: Vec<Monomial>) -> Self {
		Polynomial { monomials }
	}

	// termwise derivative; drops zero terms implicitly if callers want to filter later.
	pub fn derivative(&self) -> Polynomial {
		Polynomial {
			monomials: self
				.monomials
				.iter()
				.map(|m| m.derivative())
				.collect(),
		}
	}

	// termwise integral; propagates the first error (e.g., exponent == -1).
	pub fn integral(&self) -> Result<Polynomial, String> {
		let mut out = Vec::with_capacity(self.monomials.len());
		for m in &self.monomials {
			out.push(m.integral()?);
		}
		Ok(Polynomial { monomials: out })
	}
}

