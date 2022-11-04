//! JS Number parsing.

use std::str::FromStr;

fn split_into_radix_and_number(num: &str) -> (u32, String) {
    match num.get(0..2) {
        Some("0x") | Some("0X") => (16, num.get(2..).unwrap().replace('_', "")),
        Some("0b") | Some("0B") => (2, num.get(2..).unwrap().replace('_', "")),
        Some("0o") | Some("0O") => (8, num.get(2..).unwrap().replace('_', "")),
        _ => (10, num.replace('_', "")),
    }
}

/// Parse a js number as a string into a number.
pub fn parse_js_number(num: &str) -> Option<f64> {
    let (mut radix, raw) = split_into_radix_and_number(num);

    // account for legacy octal literals
    if raw.starts_with('0') && !raw.contains(['8', '9']) {
        radix = 8
    }

    if radix == 10 {
        f64::from_str(&raw).ok()
    } else {
        i64::from_str_radix(&raw, radix).map(|num| num as f64).ok()
    }
}

#[cfg(test)]
mod tests {
    use rome_js_factory::syntax::{JsNumberLiteralExpression, JsSyntaxKind::*};
    use rome_js_factory::JsSyntaxTreeBuilder;
    use rome_rowan::AstNode;

    fn assert_float(literal: &str, value: f64) {
        let mut tree_builder = JsSyntaxTreeBuilder::new();
        tree_builder.start_node(JS_NUMBER_LITERAL_EXPRESSION);
        tree_builder.token(JS_NUMBER_LITERAL, literal);
        tree_builder.finish_node();

        let node = tree_builder.finish();
        let number_literal = JsNumberLiteralExpression::cast(node).unwrap();
        assert_eq!(number_literal.as_number(), Some(value))
    }

    #[test]
    fn base_10_float() {
        assert_float("1234", 1234.0);
        assert_float("0", 0.0);
        assert_float("9e999", f64::INFINITY);
        assert_float("9e-999", 0.0);
    }

    #[test]
    fn base_16_float() {
        assert_float("0xFF", 255.0);
        assert_float("0XFF", 255.0);
        assert_float("0x0", 0.0);
        assert_float("0xABC", 2748.0);
        assert_float("0XABC", 2748.0);
    }

    #[test]
    fn base_2_float() {
        assert_float("0b0000", 0.0);
        assert_float("0B0000", 0.0);
        assert_float("0b11111111", 255.0);
        assert_float("0B11111111", 255.0);
    }

    #[test]
    fn base_8_float() {
        assert_float("0o77", 63.0);
        assert_float("0O77", 63.0);
        assert_float("0o0", 0.0);
        assert_float("0O0", 0.0);
    }

    #[test]
    fn base_8_legacy_float() {
        assert_float("051", 41.0);
        assert_float("058", 58.0);
    }
}
