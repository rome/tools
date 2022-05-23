use crate::options::QuoteStyle;
use crate::utils::match_ancestors;
use crate::{
    if_group_breaks, if_group_fits_on_single_line, soft_line_break, token, Formatter,
    JsFormatOptions,
};
use rome_formatter::{format_elements, space_token, FormatElement};
use rome_js_syntax::kind::JsSyntaxKind;
use rome_js_syntax::{
    JsAnyExpression, JsAnyLiteralExpression, JsLanguage, JsxAnyChild, JsxChildList,
};
use rome_rowan::SyntaxNode;

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

pub enum WrapState {
    NoWrap,
    WrapOnBreak,
    AlwaysWrap,
}

/// Takes in a syntax node because we need to handle both JsxElement and JsxSelfClosingElement
pub fn should_wrap_element_in_parens(node: &SyntaxNode<JsLanguage>) -> WrapState {
    let mut ancestors = node.ancestors();
    // We skip 1 for the current node
    ancestors.next();

    ancestors
        .next()
        .map(|parent| {
            let kind = parent.kind();
            if kind == JsSyntaxKind::JS_PARENTHESIZED_EXPRESSION {
                return should_wrap_element_in_parens(&parent);
            }
            // If our parent is one of the following kinds, we do not need to wrap
            // the element in parentheses.

            match kind {
                JsSyntaxKind::JS_ARRAY_EXPRESSION
                | JsSyntaxKind::JSX_ATTRIBUTE
                | JsSyntaxKind::JSX_ELEMENT
                | JsSyntaxKind::JSX_EXPRESSION_CHILD
                | JsSyntaxKind::JSX_FRAGMENT
                | JsSyntaxKind::JS_EXPRESSION_STATEMENT
                | JsSyntaxKind::JS_CALL_ARGUMENT_LIST
                | JsSyntaxKind::JS_CONDITIONAL_EXPRESSION => WrapState::NoWrap,
                JsSyntaxKind::JS_STATIC_MEMBER_EXPRESSION
                | JsSyntaxKind::JS_COMPUTED_MEMBER_EXPRESSION => WrapState::AlwaysWrap,
                _ => WrapState::WrapOnBreak,
            }
        })
        .unwrap_or(WrapState::NoWrap)
}

/// This is a very special situation where we're returning a JsxElement
/// from an arrow function that's passed as an argument to a function,
/// which is itself inside a JSX expression child.
///
/// If you're wondering why this is the only other case, it's because
/// Prettier defines it to be that way.
pub fn is_jsx_inside_arrow_function_inside_call_inside_expression_child(
    node: &SyntaxNode<JsLanguage>,
) -> bool {
    // ```jsx
    //  let bar = <div>
    //    {foo(() => <div> the quick brown fox jumps over the lazy dog </div>)}
    //  </div>;
    // ```
    match_ancestors(
        node,
        vec![
            None,
            Some(Box::new(|syntax: SyntaxNode<JsLanguage>| {
                syntax.kind() == JsSyntaxKind::JS_ARROW_FUNCTION_EXPRESSION
            })),
            Some(Box::new(|syntax: SyntaxNode<JsLanguage>| {
                syntax.kind() == JsSyntaxKind::JS_CALL_EXPRESSION
            })),
            Some(Box::new(|syntax: SyntaxNode<JsLanguage>| {
                syntax.kind() == JsSyntaxKind::JSX_EXPRESSION_CHILD
            })),
        ],
    )
}
