use crate::JsRuleAction;
use rome_analyze::{context::RuleContext, declare_rule, ActionCategory, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_factory::make;
use rome_js_syntax::{JsLiteralMemberName, JsSyntaxKind, JsSyntaxToken};
use rome_rowan::{AstNode, BatchMutationExt};
use std::str::FromStr;

declare_rule! {
    /// Disallow number literal object member names which are not base10 or uses underscore as separator
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// ({ 0x1: 1 });
    /// ({ 11_1.11: "ee" });
    /// ({ 0o1: 1 });
    /// ({ 1n: 1 });
    /// ({ 11_1.11: "ee" });
    /// ```
    ///
    /// ## Valid
    ///
    /// ```js
    /// ({ 0: "zero" });
    /// ({ 3.1e12: "12" });
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
    Binary {
        value: String,
        big_int: bool,
    },
    Decimal {
        value: String,
        big_int: bool,
        underscore: bool,
    },
    Octal {
        value: String,
        big_int: bool,
    },
    Hexadecimal {
        value: String,
        big_int: bool,
    },
    FloatingPoint {
        value: String,
        exponent: bool,
        underscore: bool,
    },
}

pub struct NumberLiteralError;

impl TryFrom<JsSyntaxToken> for NumberLiteral {
    type Error = NumberLiteralError;

    fn try_from(token: JsSyntaxToken) -> Result<Self, Self::Error> {
        match token.kind() {
            JsSyntaxKind::JS_NUMBER_LITERAL | JsSyntaxKind::JS_BIG_INT_LITERAL => {
                let chars: Vec<char> = token.to_string().chars().collect();
                let mut value = String::new();

                let mut is_first_char_zero: bool = false;
                let mut is_second_char_a_letter: Option<char> = None;
                let mut contains_dot: bool = false;
                let mut exponent: bool = false;
                let mut largest_digit: char = '0';
                let mut underscore: bool = false;
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

					if chars[i] == 'e' || chars[i] == 'E' {
                        exponent = true;
                    }

                    if i == 1 && chars[i].is_alphabetic() && exponent == false {
                        is_second_char_a_letter = Some(chars[i]);
                        continue;
                    }

                    if chars[i] == '_' {
                        underscore = true;
                        continue;
                    }

                    if chars[i] == '.' {
                        contains_dot = true;
                    }

                    if largest_digit < chars[i] {
                        largest_digit = chars[i];
                    }

                    value.push(chars[i])
                }

                if contains_dot {
                    return Ok(Self::FloatingPoint {
                        value,
                        exponent,
                        underscore,
                    });
                };
                if !is_first_char_zero {
                    return Ok(Self::Decimal {
                        value,
                        big_int,
                        underscore,
                    });
                };

                match is_second_char_a_letter {
                    Some('b' | 'B') => return Ok(Self::Binary { value, big_int }),
                    Some('o' | 'O') => return Ok(Self::Octal { value, big_int }),
                    Some('x' | 'X') => return Ok(Self::Hexadecimal { value, big_int }),
                    _ => (),
                }

                if largest_digit < '8' {
                    return Ok(Self::Octal { value, big_int });
                }

                Ok(Self::Decimal {
                    value,
                    big_int,
                    underscore,
                })
            }
            _ => Err(NumberLiteralError),
        }
    }
}

impl NumberLiteral {
    fn value(&self) -> &String {
        match self {
            Self::Decimal { value, .. } => value,
            Self::Binary { value, .. } => value,
            Self::FloatingPoint { value, .. } => value,
            Self::Octal { value, .. } => value,
            Self::Hexadecimal { value, .. } => value,
        }
    }
}

impl NumberLiteral {
    fn to_base_ten(&self) -> Option<f64> {
        match self {
            Self::Binary { value, .. } => i64::from_str_radix(value, 2).map(|num| num as f64).ok(),
            Self::Decimal { value, .. } | Self::FloatingPoint { value, .. } => {
                f64::from_str(value).ok()
            }
            Self::Octal { value, .. } => i64::from_str_radix(value, 7).map(|num| num as f64).ok(),
            Self::Hexadecimal { value, .. } => {
                i64::from_str_radix(value, 16).map(|num| num as f64).ok()
            }
        }
    }
}

impl Rule for UseSimpleNumberKeys {
    type Query = Ast<JsLiteralMemberName>;
    type State = NumberLiteral;
    type Signals = Vec<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let mut signals: Self::Signals = Vec::new();
        let node = ctx.query();

        if let Ok(token) = node.value() {
            let number_literal = NumberLiteral::try_from(token).ok();

            if let Some(number_literal) = number_literal {
                match number_literal {
                    NumberLiteral::Decimal { big_int: true, .. }
                    | NumberLiteral::Decimal {
                        underscore: true, ..
                    } => signals.push(number_literal),
                    NumberLiteral::FloatingPoint {
                        underscore: true, ..
                    } => signals.push(number_literal),
                    NumberLiteral::Binary { .. } => signals.push(number_literal),
                    NumberLiteral::Hexadecimal { .. } => signals.push(number_literal),
                    NumberLiteral::Octal { .. } => signals.push(number_literal),
                    _ => (),
                }
            }
        }

        signals
    }

    fn diagnostic(
        _ctx: &RuleContext<Self>,
        number_literal: &Self::State,
    ) -> Option<RuleDiagnostic> {
        let title = match number_literal {
            NumberLiteral::Decimal { big_int: true, .. } => "Bigint is not allowed",
            NumberLiteral::Decimal {
                underscore: true, ..
            } => "Number literal with underscore is not allowed",
            NumberLiteral::FloatingPoint {
                underscore: true, ..
            } => "Number literal with underscore is not allowed",
            NumberLiteral::Binary { .. } => "Number literal in binary format is not allowed",
            NumberLiteral::Hexadecimal { .. } => {
                "Number literal in hexadecimal format is not allowed"
            }
            NumberLiteral::Octal { .. } => "Number literal in octal format is not allowed",
            _ => "",
        };

        let diagnostic =
            RuleDiagnostic::new(rule_category!(), _ctx.query().range(), title.to_string());

        Some(diagnostic)
    }

    fn action(ctx: &RuleContext<Self>, number_literal: &Self::State) -> Option<JsRuleAction> {
        let mut mutation = ctx.root().begin();
        let node = ctx.query();

        if let Ok(token) = node.value() {
            match number_literal {
                NumberLiteral::Binary { .. }
                | NumberLiteral::Octal { .. }
                | NumberLiteral::Hexadecimal { .. } => mutation.replace_token(
                    token,
                    make::js_number_literal(number_literal.to_base_ten()?),
                ),
                NumberLiteral::FloatingPoint { .. } | NumberLiteral::Decimal { .. } => {
                    mutation.replace_token(token, make::js_number_literal(number_literal.value()));
                }
            };

            return Some(JsRuleAction {
                category: ActionCategory::QuickFix,
                applicability: Applicability::Always,
                message: markup! ("Remove "{ node.to_string() }).to_owned(),
                mutation,
            });
        }

        None
    }
}
