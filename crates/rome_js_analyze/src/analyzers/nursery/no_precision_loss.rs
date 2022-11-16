use std::num::IntErrorKind;

use rome_analyze::context::RuleContext;
use rome_analyze::{declare_rule, Ast, Rule, RuleDiagnostic};
use rome_console::markup;

use rome_js_syntax::numbers::parse_js_number;
use rome_js_syntax::JsNumberLiteralExpression;
use rome_rowan::AstNode;

declare_rule! {
    /// Promotes the use of `.flatMap()` when `map().flat()` are used together.
    ///
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const array = ["split", "the text", "into words"];
    /// array.map(sentence => sentence.split(' ')).flat();
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const array = ["split", "the text", "into words"];
    /// array.map(sentence => sentence.split(' ')).flat(1);
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const array = ["split", "the text", "into words"];
    /// array.map(sentence => sentence.split(' ')).flat(2);
    /// ```
    ///
    pub(crate) NoPrecisionLoss {
        version: "11.0.0",
        name: "noPrecisionLoss",
        recommended: false,
    }
}

impl Rule for NoPrecisionLoss {
    type Query = Ast<JsNumberLiteralExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        if is_precision_lost(node)? {
            Some(())
        } else {
            None
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();

        Some(RuleDiagnostic::new(
            rule_category!(),
            node.syntax().text_trimmed_range(),
            markup! { "This number literal will lose precision at runtime." },
        ))
    }
}

fn is_precision_lost(node: &JsNumberLiteralExpression) -> Option<bool> {
    let token = node.value_token().ok()?;
    let num = token.text_trimmed();

    let num = num.replace('_', "").replace('E', "e");
    let (radix, num) = get_radix_and_num(&num);
    if radix == 10 {
        is_precision_lost_base_10(num)
    } else {
        Some(is_precision_lost_base_other(num, radix))
    }
}

fn get_radix_and_num(num: &str) -> (u32, &str) {
    let mut chars = num.chars();
    match (chars.next(), chars.next()) {
        (Some('0'), Some('x' | 'X')) => (16, &num[2..]),
        (Some('0'), Some('o' | 'O')) => (8, &num[2..]),
        (Some('0'), Some('b' | 'B')) => (2, &num[2..]),
        (Some('0'), Some('0'..='7')) if chars.all(|it| matches!(it, '0'..='7')) => (8, &num[1..]),
        _ => (10, num),
    }
}

fn is_precision_lost_base_10(num: &str) -> Option<bool> {
    let normalized = NormalizedNum::new(num);
    if normalized.is_zero() {
        return Some(false);
    }
    let precision = normalized.precision();
    if precision > 100 {
        return Some(true);
    }
    let parsed = parse_js_number(num)?;
    let stored_num = format!("{:.*e}", precision - 1, parsed);
    Some(stored_num != normalized.to_scientific())
}

fn is_precision_lost_base_other(num: &str, radix: u32) -> bool {
    let parsed = match i64::from_str_radix(num, radix) {
        Ok(x) => x,
        Err(e) => {
            return matches!(
                e.kind(),
                IntErrorKind::PosOverflow | IntErrorKind::NegOverflow
            )
        }
    };
    const MAX_SAFE_INTEGER: i64 = 2_i64.pow(53) - 1;
    const MIN_SAFE_INTEGER: i64 = -2_i64.pow(53) + 1;
    parsed < MIN_SAFE_INTEGER || parsed > MAX_SAFE_INTEGER
}

fn remove_leading_zeros(num: &str) -> &str {
    num.trim_start_matches('0')
}

fn remove_trailing_zeros(num: &str) -> &str {
    num.trim_end_matches('0')
}

#[derive(Debug)]
/// Normalized number in the form `0.{digits}.{digits_rest}e{exponent}`
struct NormalizedNum<'a> {
    digits: &'a str,
    digits_rest: &'a str,
    exponent: isize,
}

impl NormalizedNum<'_> {
    fn new(num: &str) -> NormalizedNum<'_> {
        let num = remove_leading_zeros(num);
        let mut split = num.splitn(2, 'e');
        let mantissa = split.next().unwrap();
        let exponent = split.next();
        let mut mantissa_parts = mantissa.splitn(2, '.');

        let mut normalized = match (mantissa_parts.next().unwrap(), mantissa_parts.next()) {
            ("", Some(fraction)) => {
                let digits = remove_leading_zeros(fraction);
                NormalizedNum {
                    digits,
                    digits_rest: "",
                    exponent: digits.len() as isize - fraction.len() as isize,
                }
            }
            (integer, Some(fraction)) => NormalizedNum {
                digits: integer,
                digits_rest: fraction,
                exponent: integer.len() as isize,
            },
            (integer, None) => {
                let digits = remove_trailing_zeros(integer);
                NormalizedNum {
                    digits,
                    digits_rest: "",
                    exponent: integer.len() as isize,
                }
            }
        };

        if let Some(exponent) = exponent.and_then(|it| isize::from_str_radix(it, 10).ok()) {
            normalized.exponent += exponent;
        }

        normalized
    }

    fn to_scientific(&self) -> String {
        format!(
            "{}.{}{}e{}",
            &self.digits[..1],
            &self.digits[1..],
            self.digits_rest,
            self.exponent - 1
        )
    }

    fn is_zero(&self) -> bool {
        self.digits.is_empty()
    }

    fn precision(&self) -> usize {
        self.digits.len() + self.digits_rest.len()
    }
}
