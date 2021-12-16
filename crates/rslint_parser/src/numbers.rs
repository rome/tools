//! JS Number parsing.

use lexical::parse_radix;

pub use num_bigint::BigInt;

fn split_into_radix_and_number(num: &str) -> (u32, String) {
	match num.get(0..2) {
		Some("0x") | Some("0X") => (16, num.get(2..).unwrap().replace("_", "")),
		Some("0b") | Some("0B") => (2, num.get(2..).unwrap().replace("_", "")),
		Some("0o") | Some("0O") => (8, num.get(2..).unwrap().replace("_", "")),
		_ => (10, num.replace("_", "")),
	}
}

/// Parse a js number as a string into a number.
pub fn parse_js_number(num: &str) -> Option<f64> {
	let (radix, raw) = split_into_radix_and_number(num);

	if radix == 10 && raw.starts_with('0') {
		// account for legacy octal literals
		if let Ok(parsed) = parse_radix::<f64, _>(raw.as_bytes(), 8) {
			return Some(parsed);
		}
	}

	parse_radix::<f64, _>(raw.as_bytes(), radix as u8).ok()
}

/// Parse a big int number as a string into a number.
pub fn parse_js_big_int(num: &str) -> Option<BigInt> {
	let (radix, raw) = split_into_radix_and_number(num);

	let raw = if raw.get(raw.len() - 1..raw.len()) == Some("n") {
		raw.split_at(raw.len() - 1).0.to_string()
	} else {
		raw
	};
	BigInt::parse_bytes(raw.as_bytes(), radix)
}

#[cfg(test)]
mod tests {
	use crate::{
		ast::{JsAnyExpression, JsAnyLiteralExpression},
		parse_expr,
	};
	use num_bigint::ToBigInt;

	macro_rules! assert_float {
		($literal:literal, $value:expr) => {
			let parsed = parse_expr($literal, 0);
			match parsed.tree().expression().unwrap() {
				JsAnyExpression::JsAnyLiteralExpression(
					JsAnyLiteralExpression::JsNumberLiteralExpression(literal),
				) => {
					assert_eq!(literal.as_number(), Some($value));
				}
				_ => panic!(
					"Parsed expression is not a number literal. Expr:\n{:#?}",
					parsed.syntax()
				),
			}
		};
	}

	macro_rules! assert_bigint {
		($literal:literal, $value:expr) => {
			let parsed = parse_expr($literal, 0);
			match parsed.tree().expression().unwrap() {
				JsAnyExpression::JsAnyLiteralExpression(
					JsAnyLiteralExpression::JsBigIntLiteralExpression(literal),
				) => {
					let val = ($value as u64).to_bigint();
					assert_eq!(literal.as_number(), val);
				}
				_ => {
					panic!(
						"Parsed expression is not a big int literal. Expr:\n{:#?}",
						parsed.syntax()
					);
				}
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
