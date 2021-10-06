//! JS Number parsing.

use lexical::parse_radix;

pub use num_bigint::BigInt;

#[derive(Debug, Clone, PartialEq)]
pub enum JsNum {
	Float(f64),
	BigInt(BigInt),
}

/// Parse a js number as a string into a number.  
pub fn parse_js_num(num: String) -> Option<JsNum> {
	let (radix, mut raw) = match num.get(0..2) {
		Some("0x") | Some("0X") => (16, num.get(2..).unwrap().replace("_", "")),
		Some("0b") | Some("0B") => (2, num.get(2..).unwrap().replace("_", "")),
		Some("0o") | Some("0O") => (8, num.get(2..).unwrap().replace("_", "")),
		_ => (10, num.as_str().replace("_", "")),
	};

	if radix == 10 && raw.starts_with('0') {
		// account for legacy octal literals
		if let Ok(parsed) = parse_radix(raw.as_bytes(), 8) {
			return Some(JsNum::Float(parsed));
		}
	}

	let bigint = if raw.get(raw.len() - 1..raw.len()) == Some("n") {
		raw = raw.split_at(raw.len() - 1).0.to_string();
		true
	} else {
		false
	};

	if bigint {
		Some(JsNum::BigInt(BigInt::parse_bytes(raw.as_bytes(), radix)?))
	} else {
		Some(JsNum::Float(
			parse_radix::<f64, _>(raw.as_bytes(), radix as u8).ok()?,
		))
	}
}

#[cfg(test)]
mod tests {
	use crate::{
		ast::{Expr, LiteralKind},
		parse_expr,
	};
	use num_bigint::ToBigInt;

	macro_rules! assert_float {
		($literal:literal, $value:expr) => {
			let parsed = parse_expr($literal, 0);
			if let Expr::Literal(literal) = parsed.tree() {
				assert_eq!(literal.as_number(), Some($value));
			} else {
				panic!("Parsed expression is not a literal");
			}
		};
	}

	macro_rules! assert_bigint {
		($literal:literal, $value:expr) => {
			let parsed = parse_expr($literal, 0);
			if let Expr::Literal(literal) = parsed.tree() {
				let val = ($value as u64).to_bigint().unwrap();
				assert_eq!(literal.kind(), LiteralKind::BigInt(val));
			} else {
				panic!("Parsed expression is not a literal");
			}
		};
	}

	#[test]
	fn base_10_float() {
		assert_float!("1234", 1234.0);
		assert_float!("0", 0.0);
		assert_float!("9e999", f64::INFINITY);
		assert_float!("9e-999", 0.0);
	}

	#[test]
	fn base_16_float() {
		assert_float!("0xFF", 255.0);
		assert_float!("0XFF", 255.0);
		assert_float!("0x0", 0.0);
		assert_float!("0xABC", 2748.0);
		assert_float!("0XABC", 2748.0);
	}

	#[test]
	fn base_2_float() {
		assert_float!("0b0000", 0.0);
		assert_float!("0B0000", 0.0);
		assert_float!("0b11111111", 255.0);
		assert_float!("0B11111111", 255.0);
	}

	#[test]
	fn base_8_float() {
		assert_float!("0o77", 63.0);
		assert_float!("0O77", 63.0);
		assert_float!("0o0", 0.0);
		assert_float!("0O0", 0.0);
	}

	#[test]
	fn base_8_legacy_float() {
		assert_float!("051", 41.0);
		assert_float!("058", 58.0);
	}

	#[test]
	fn base_10_bigint() {
		assert_bigint!("1010n", 1010);
		assert_bigint!("0n", 0);
		assert_bigint!("9007199254740991n", 9007199254740991);
	}

	#[test]
	fn base_16_bigint() {
		assert_bigint!("0xffn", 255);
		assert_bigint!("0XFFn", 255);
		assert_bigint!("0x1fffffffffffffn", 9007199254740991);
		assert_bigint!("0X1fffffffffffffn", 9007199254740991);
	}

	#[test]
	fn base_2_bigint() {
		assert_bigint!("0b0n", 0);
		assert_bigint!("0B0n", 0);
		assert_bigint!(
			"0b11111111111111111111111111111111111111111111111111111n",
			9007199254740991
		);
		assert_bigint!(
			"0B11111111111111111111111111111111111111111111111111111n",
			9007199254740991
		);
	}
}
