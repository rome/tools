//! JS Number parsing.

use std::str::FromStr;

/// Split given string into radix and number string.
///
/// It also removes any underscores.
pub fn split_into_radix_and_number(num: &str) -> (u32, String) {
    let (radix, raw) = parse_js_number_prefix(num).unwrap_or((10, num));
    let raw = raw.replace('_', "");
    (radix, raw)
}

fn parse_js_number_prefix(num: &str) -> Option<(u32, &str)> {
    let mut chars = num.chars();
    let c = chars.next()?;
    if c != '0' {
        return None;
    }
    Some(match chars.next()? {
        'x' | 'X' => (16, chars.as_str()),
        'o' | 'O' => (8, chars.as_str()),
        'b' | 'B' => (2, chars.as_str()),
        // Legacy octal literals
        '0'..='7' if !chars.as_str().contains(['8', '9']) => (8, &num[1..]),
        _ => return None,
    })
}

/// Parse a js number as a string into a number.
pub fn parse_js_number(num: &str) -> Option<f64> {
    let (radix, raw) = split_into_radix_and_number(num);

    if radix == 10 {
        f64::from_str(&raw).ok()
    } else {
        i64::from_str_radix(&raw, radix).map(|num| num as f64).ok()
    }
}

#[cfg(test)]
mod tests {
    use super::split_into_radix_and_number;
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

    fn assert_split(raw: &str, expected_radix: u32, expected_num: &str) {
        let (radix, num) = split_into_radix_and_number(raw);
        assert_eq!(radix, expected_radix);
        assert_eq!(num, expected_num);
    }

    #[test]
    fn split_hex() {
        assert_split("0x12", 16, "12");
        assert_split("0X12", 16, "12");
        assert_split("0x1_2", 16, "12");
        assert_split("0X1_2", 16, "12");
    }

    #[test]
    fn split_binary() {
        assert_split("0b01", 2, "01");
        assert_split("0b01", 2, "01");
        assert_split("0b0_1", 2, "01");
        assert_split("0b0_1", 2, "01");
    }

    #[test]
    fn split_octal() {
        assert_split("0o12", 8, "12");
        assert_split("0o12", 8, "12");
        assert_split("0o1_2", 8, "12");
        assert_split("0o1_2", 8, "12");
    }

    #[test]
    fn split_legacy_octal() {
        assert_split("012", 8, "12");
        assert_split("012", 8, "12");
        assert_split("01_2", 8, "12");
        assert_split("01_2", 8, "12");
    }

    #[test]
    fn split_legacy_decimal() {
        assert_split("1234", 10, "1234");
        assert_split("1234", 10, "1234");
        assert_split("12_34", 10, "1234");
        assert_split("12_34", 10, "1234");
    }
}
