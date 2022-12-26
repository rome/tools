use rome_analyze::{
    context::RuleContext, declare_rule, Rule, Ast, RuleDiagnostic
};
use rome_js_syntax::JsLiteralMemberName;
use rome_js_syntax::JsSyntaxKind;
use rome_js_syntax::JsSyntaxToken;
use rome_rowan::AstNode;

declare_rule! {
    /// Put your description here
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// var a = 1;
    /// a = 2;
    /// ```
    ///
    /// ## Valid
    ///
    /// ```js
    /// var a = 1;
    /// ```
    ///
    pub(crate) UseSimpleNumberKeys {
        version: "next",
        name: "useSimpleNumberKeys",
        recommended: false,
    }
}

#[derive(Clone)]
pub enum NumberLiteral {
	Binary{value: String, big_int: bool},
	Decimal{value: String, big_int: bool, dashed: bool},
	Octal{value: String, big_int: bool},
	Hexadecimal{value: String, big_int: bool},
	FloatingPoint{value: String, exponent: bool, dashed: bool}
}

pub struct NumberLiteralError;

impl TryFrom<JsSyntaxToken> for NumberLiteral {

	type Error = NumberLiteralError;


	fn try_from(token: JsSyntaxToken) -> Result<Self, Self::Error> {
		match token.kind() {
			JsSyntaxKind::JS_NUMBER_LITERAL |  JsSyntaxKind::JS_BIG_INT_LITERAL =>  {
				let chars: Vec<char> = token.to_string().chars().collect();
				let mut value = String::new();

				let mut is_first_char_zero: bool = false;
				let mut is_second_char_a_letter: Option<char> = None;
				let mut contains_dot: bool = false;
				let mut exponent: bool = false;
				let mut largest_digit: char = '0';
				let mut dashed: bool = false;
				let mut big_int: bool = false;


				for i in 0..chars.len() {
					if i == 0 && chars[i] == '0' && chars.len() > 1 {
						is_first_char_zero = true;
						continue;
					}

					if chars[i] == 'n' {
						big_int = true;
						break;
					}

					if i == 1 && chars[i].is_alphabetic() {
						is_second_char_a_letter = Some(chars[i]);
						continue;
					}

					if chars[i] == '_' {
						dashed = true;
					}

					if chars[i] == '.' {
						contains_dot = true;
					}

					if contains_dot && (chars[i] == 'e' || chars[i] == 'E') {
						exponent = true;
					}

					if largest_digit < chars[i] {
						largest_digit = chars[i];
					}

					value.push(chars[i])
				}

				if contains_dot {return Ok(Self::FloatingPoint{value, exponent, dashed})};
				if !is_first_char_zero {return Ok(Self::Decimal{value, big_int, dashed})};

				match is_second_char_a_letter {
					Some('b' | 'B') => {return Ok(Self::Binary{value, big_int})},
					Some('o' | 'O') => {return Ok(Self::Octal{value, big_int})},
					Some('x' | 'X' ) => {return Ok(Self::Hexadecimal{value, big_int})},
					_ => ()
				}

				if largest_digit < '8' {
					return Ok(Self::Octal{value, big_int})
				}

				Ok(Self::Decimal{value, big_int, dashed})

			}
			_ => Err(NumberLiteralError)
		}
	}
}

impl NumberLiteral {
	fn value(self: &Self) -> &String {
		return match self {
			Self::Decimal { value, .. } => value,
			Self::Binary { value, .. } => value,
			Self::FloatingPoint { value,.. } => value,
			Self::Octal { value, .. } => value,
			Self::Hexadecimal { value, .. } => value
		}

	}
}

impl NumberLiteral {
	fn to_base_ten(self: &Self) -> String {
		let result = match self {
			Self::Binary { value, .. } => {
				i32::from_str_radix(value, 2).ok()
			}
			Self::Octal { value,.. } => {
				i32::from_str_radix(value, 7).ok()
			}
			Self::Hexadecimal { value, ..} => {i32::from_str_radix(value, 16).ok()},
			_ => None
		};

		match result {
			Some(value) => {return value.to_string();},
			None => {return self.value().to_string()}
		}
	}
}

impl Rule for UseSimpleNumberKeys {
    type Query = Ast<JsLiteralMemberName>;
    type State = NumberLiteral;
    type Signals =  Vec<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
		let mut signals: Self::Signals = Vec::new();
		let node = ctx.query();

		if let Some(token) = node.value().ok() {

			let number_literal = NumberLiteral::try_from(token).ok();

			match number_literal {
				Some(number_literal) => {
					match number_literal {
						NumberLiteral::Decimal {  big_int: true, .. } | NumberLiteral::Decimal {  dashed: true, .. } => {signals.push(number_literal)},
						NumberLiteral::FloatingPoint {  dashed: true, .. } => {signals.push(number_literal)},
						NumberLiteral::Binary { .. } => {signals.push(number_literal)}
						NumberLiteral::Hexadecimal { .. } => {signals.push(number_literal)}
						NumberLiteral::Octal { .. } => {signals.push(number_literal)}
						_ => ()
					}
				},
				None => (),
			}
		}

		signals
    }

	fn diagnostic(
        _ctx: &RuleContext<Self>,
		number_literal: &Self::State
    ) -> Option<RuleDiagnostic> {


		let title = match number_literal {
			NumberLiteral::Decimal { big_int: true, .. } => { "Bigint is not allowed"},
			NumberLiteral::Decimal { dashed: true, .. } => { "Dashed number literal is not allowed"},
			NumberLiteral::FloatingPoint { dashed: true, .. } => { "Dashed number literal is not allowed" },
			NumberLiteral::Binary { .. } => { "Number literal in binary format is not allowed" },
			NumberLiteral::Hexadecimal { .. } => {  "Number literal in hexadecimal format is not allowed"},
			NumberLiteral::Octal { .. } => { "Number literal in octal format is not allowed"},
			_ => {""}
		};

		let diagnostic = RuleDiagnostic::new(
            rule_category!(),
				_ctx.query().range(),
                format!("{}", title),
            );

		Some(diagnostic)
	}

}
