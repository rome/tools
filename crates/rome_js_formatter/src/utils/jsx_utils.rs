use crate::options::QuoteStyle;
use crate::{
    if_group_breaks, if_group_fits_on_single_line, soft_line_break, token, Formatter,
    JsFormatOptions,
};
use rome_formatter::{format_elements, space_token, FormatElement};
use rome_js_syntax::kind::JsSyntaxKind;
use rome_js_syntax::{
    JsAnyExpression, JsAnyLiteralExpression, JsxAnyChild, JsxChildList, JsxElement,
};
use rome_rowan::AstNode;

pub fn jsx_space(formatter: &Formatter<JsFormatOptions>) -> FormatElement {
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
        if let Some(JsAnyExpression::JsAnyLiteralExpression(
            JsAnyLiteralExpression::JsStringLiteralExpression(string_literal_expr),
        )) = expr_child.expression()
        {
            if let Ok(token) = string_literal_expr.value_token() {
                return token.text() == " ";
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

pub fn should_wrap_element_in_parens(element: &JsxElement) -> bool {
    let mut ancestors = element.syntax().ancestors();
    // We skip one because all elements are wrapped in JS_TAG_EXRESSION
    ancestors.next();

    ancestors
        .next()
        .map(|parent| {
            !matches!(
                parent.kind(),
                JsSyntaxKind::JS_ARRAY_EXPRESSION
                    | JsSyntaxKind::JSX_ATTRIBUTE
                    | JsSyntaxKind::JSX_ELEMENT
                    | JsSyntaxKind::JSX_EXPRESSION_CHILD
                    | JsSyntaxKind::JSX_FRAGMENT
                    | JsSyntaxKind::JS_EXPRESSION_STATEMENT
                    | JsSyntaxKind::JS_CALL_EXPRESSION
                    | JsSyntaxKind::JS_CONDITIONAL_EXPRESSION
                    | JsSyntaxKind::JS_PARENTHESIZED_EXPRESSION
            )
        })
        .unwrap_or(true)
}
