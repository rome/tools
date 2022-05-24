use crate::options::QuoteStyle;
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

/// Creates either a space using an expression child and a string literal,
/// or a regular space, depending on whether the group breaks or not.
///
/// ```jsx
///  <div> Winter Light </div>;
///
///  <div>
///    {" "}Winter Light
///    Through A Glass Darkly
///    The Silence
///    Seventh Seal
///    Wild Strawberries
///  </div>
/// ```
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

/// Detects if the child is a JSX whitespace expression, i.e. `{" "}`
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

/// Meaningful JSX text is defined to be text that has either non-whitespace
/// characters, or does not contain a newline. Whitespace is defined as ASCII
/// whitespace.
///
/// ```
/// use rome_js_formatter::utils::jsx_utils::is_meaningful_jsx_text;
///
/// assert_eq!(is_meaningful_jsx_text("     \t\r   "), true);
/// assert_eq!(is_meaningful_jsx_text("     \n\r   "), false);
/// assert_eq!(is_meaningful_jsx_text("  Alien   "), true);
/// assert_eq!(is_meaningful_jsx_text(""), true);
/// ```
pub fn is_meaningful_jsx_text(text: &str) -> bool {
    let mut has_newline = false;
    for c in text.chars() {
        // If there is a non-whitespace character
        if !WHITESPACE.contains(&c) {
            return true;
        }
        if c == '\n' {
            has_newline = true;
        }
    }

    !has_newline
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

/// Checks if a JSX Element should be wrapped in parentheses. Returns a WrapState which
/// indicates that an element should always be wrapped in parentheses, should be wrapped
/// only when it's line broken, or should not be wrapped at all.
///
/// For instance, an element nested in a let binding should be wrapped in parentheses
/// when it is line broken:
/// ```jsx
///  let component = <div> La Haine dir. Mathieu Kassovitz </div>;
///
///  let component = (
///   <div> Uncle Boonmee Who Can Recall His Past Lives dir. Apichatpong Weerasethakul </div>
///  );
/// ```
///
/// An element inside a static member expression should always be wrapped:
/// ```jsx
/// (<div>Badlands</div>).property
/// ```
///
/// An element that is another element's attribute should never be wrapped:
/// ```jsx
///  <Route path="/" component={<HomePage />} />
/// ```
///
pub fn should_wrap_element_in_parens(node: &SyntaxNode<JsLanguage>) -> WrapState {
    // We skip the first item because the first item in ancestors is the node itself, i.e.
    // the JSX Element in this case.
    let mut ancestors = node.ancestors().skip(1);

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
///
/// ```jsx
///  let bar = <div>
///    {foo(() => <div> the quick brown fox jumps over the lazy dog </div>)}
///  </div>;
/// ```
pub fn is_jsx_inside_arrow_function_inside_call_inside_expression_child(
    node: &SyntaxNode<JsLanguage>,
) -> bool {
    // We skip the first item because the first item in ancestors is the node itself, i.e.
    // the JSX Element in this case.
    let mut ancestors = node.ancestors().skip(1);
    let required_ancestors = [
        JsSyntaxKind::JS_ARROW_FUNCTION_EXPRESSION,
        JsSyntaxKind::JS_CALL_EXPRESSION,
        JsSyntaxKind::JSX_EXPRESSION_CHILD,
    ];

    for required_ancestor in required_ancestors {
        let is_required_ancestor = ancestors
            .next()
            .map(|ancestor| ancestor.kind() == required_ancestor)
            .unwrap_or(false);
        if !is_required_ancestor {
            return false;
        }
    }

    true
}
