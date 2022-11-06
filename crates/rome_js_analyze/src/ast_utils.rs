use rome_js_syntax::{
    JsAnyExpression, JsComputedMemberExpression, JsIdentifierExpression, JsLanguage, JsName,
    JsNumberLiteralExpression, JsStaticMemberExpression, JsStringLiteralExpression, JsTemplate,
};
use rome_rowan::{match_ast, AstNode, AstNodeList, SyntaxResult};

/// Check if the given node is an identifier with given value
pub fn is_ident_eq<T>(node: &T, name: &str) -> bool
where
    T: AstNode<Language = JsLanguage>,
{
    let syntax = node.syntax();
    JsIdentifierExpression::can_cast(syntax.kind()) && syntax.text_trimmed() == name
}

/// Check if the given node is a name with given value
pub fn is_name_eq<T>(node: &T, name: &str) -> bool
where
    T: AstNode<Language = JsLanguage>,
{
    let syntax = node.syntax();
    JsName::can_cast(syntax.kind()) && syntax.text_trimmed() == name
}

/// Check if given node is a member access with given Object name and property name.
/// Both static and computed access are checked.
pub fn is_member_access_eq<T>(node: &T, object_name: &str, property_name: &str) -> bool
where
    T: AstNode<Language = JsLanguage>,
{
    fn is_normalized_ident_eq(node: SyntaxResult<JsAnyExpression>, object_name: &str) -> bool {
        node.ok()
            .and_then(remove_parentheses)
            .map(|it| is_ident_eq(&it, object_name))
            .unwrap_or(false)
    }

    match_ast! {
        match (node.syntax()) {
            JsStaticMemberExpression(x) => {
                is_normalized_ident_eq(x.object(), object_name) &&
                x.member().map(|it| is_name_eq(&it, property_name)).unwrap_or(false)
            },
            JsComputedMemberExpression(x) => {
                is_normalized_ident_eq(x.object(), object_name) &&
                x.member().map(|it| is_static_text_eq(&it, property_name)).unwrap_or(false)
            },
            _ => false
        }
    }
}

/// Check if given string value equals node's static string.
pub fn is_static_text_eq<T>(node: &T, property_name: &str) -> bool
where
    T: AstNode<Language = JsLanguage>,
{
    with_static_text(node, |t| t == property_name).unwrap_or(false)
}

/// Returns string value if node is a static string.
pub fn as_static_text<T>(node: &T) -> Option<String>
where
    T: AstNode<Language = JsLanguage>,
{
    with_static_text(node, |t| t.to_owned())
}

/// Checks if the given node is a static string and calls the given function with the string value.
pub fn with_static_text<T, F, R>(node: &T, f: F) -> Option<R>
where
    T: AstNode<Language = JsLanguage>,
    F: FnOnce(&str) -> R,
{
    match_ast! {
        match (node.syntax()) {
            JsTemplate(t) => {
                if t.tag().is_some() || t.elements().len() != 1 {
                    return None;
                }

                let e = t.elements().into_iter().next().unwrap();
                let chunk = e.as_js_template_chunk_element().unwrap();
                match chunk.template_chunk_token() {
                    Ok(c) => Some(f(c.text_trimmed())),
                    _ => None,
                }
            },
            JsStringLiteralExpression(s) => {
                match s.value_token() {
                    Ok(t) => {
                        let text = t.text_trimmed();
                        Some(f(&text[1..text.len() - 1]))
                    },
                    _ => None
                }
            },
            _ => None,
        }
    }
}

/// Get number value for the given node.
pub fn as_number<T>(node: &T) -> Option<f64>
where
    T: AstNode<Language = JsLanguage>,
{
    match_ast! {
        match (node.syntax()) {
            JsNumberLiteralExpression(n) => n.as_number(),
            _ => None
        }
    }
}

/// Recursively remove parentheses from a given expression.
pub fn remove_parentheses(mut expr: JsAnyExpression) -> Option<JsAnyExpression> {
    loop {
        match expr {
            JsAnyExpression::JsParenthesizedExpression(e) => {
                expr = e.expression().ok()?;
            }
            _ => break Some(expr),
        }
    }
}
