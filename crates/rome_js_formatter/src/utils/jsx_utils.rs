use crate::utils::format_string_literal_token;
use crate::{
    if_group_breaks, if_group_fits_on_single_line, soft_line_break, token, Formatter, Token,
};
use rome_formatter::{format_elements, space_token, FormatElement, QuoteStyle};
use rome_js_syntax::{JsAnyExpression, JsAnyLiteralExpression, JsxAnyChild, JsxChildList};

pub fn jsx_space(formatter: &Formatter) -> FormatElement {
    let jsx_space = match formatter.options().quote_style {
        QuoteStyle::Double => "{{\" \"}}",
        QuoteStyle::Single => "{{\' \'}}",
    };

    format_elements![
        if_group_breaks(format_elements![token(jsx_space), soft_line_break()]),
        if_group_fits_on_single_line(space_token())
    ]
}

pub fn is_jsx_whitespace_expression(child: JsxAnyChild) -> bool {
    if let JsxAnyChild::JsxExpressionChild(expr_child) = child {
        if let Some(expr) = expr_child.expression() {
            if let JsAnyExpression::JsAnyLiteralExpression(
                JsAnyLiteralExpression::JsStringLiteralExpression(string_literal_expr),
            ) = expr
            {
                if let Ok(token) = string_literal_expr.value_token() {
                    return token.text() == " ";
                }
            }
        }
    }

    false
}

pub fn contains_text(children: &JsxChildList) -> bool {
    for child in children {
        if let JsxAnyChild::JsxText(jsx_text) = child {
            if let Ok(token) = jsx_text.value_token() {
                if is_meaningful_jsx_text(token.text()) {
                    return true;
                }
            }
        }
    }

    false
}

pub static WHITESPACE: [char; 4] = [' ', '\n', '\t', '\r'];

pub fn is_meaningful_jsx_text(text: &str) -> bool {
    let mut has_newline = false;
    let mut has_non_whitespace = false;
    for c in text.chars() {
        // If there is a non-whitespace character
        if !WHITESPACE.contains(&c) {
            has_non_whitespace = true;
        }
        if c == '\n' {
            has_newline = true;
        }
    }

    has_non_whitespace || !has_newline
}

pub fn contains_tag(children: &JsxChildList) -> bool {
    for child in children {
        if matches!(
            child,
            JsxAnyChild::JsxElement(_)
                | JsxAnyChild::JsxSelfClosingElement(_)
                | JsxAnyChild::JsxFragment(_)
        ) {
            return true;
        }
    }

    false
}

pub fn contains_multiple_expressions(children: &JsxChildList) -> bool {
    let mut seen_expression = false;
    for child in children {
        if matches!(child, JsxAnyChild::JsxExpressionChild(_)) {
            if seen_expression {
                return true;
            }
            seen_expression = true;
        }
    }

    false
}
