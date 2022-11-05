use rome_js_syntax::{
    JsComputedMemberExpression, JsIdentifierExpression, JsLanguage, JsName,
    JsNumberLiteralExpression, JsStaticMemberExpression, JsStringLiteralExpression, JsTemplate,
};
use rome_rowan::{match_ast, AstNode, AstNodeList};

pub fn is_specific_id<T>(node: &T, name: &str) -> bool
where
    T: AstNode<Language = JsLanguage>,
{
    is_id(node) && node.syntax().text_trimmed() == name
}

pub fn is_id<T>(node: &T) -> bool
where
    T: AstNode<Language = JsLanguage>,
{
    match_ast! {
        match (node.syntax()) {
            JsIdentifierExpression(..) => true,
            JsName(..) => true,
            _ => false
        }
    }
}

pub fn is_specific_member_access<T>(node: &T, object_name: &str, property_name: &str) -> bool
where
    T: AstNode<Language = JsLanguage>,
{
    match_ast! {
        match (node.syntax()) {
            JsStaticMemberExpression(x) => {
                x.object().map(|it| is_specific_id(&it, object_name)).unwrap_or(false) &&
                x.member().map(|it| is_specific_id(&it, property_name)).unwrap_or(false)
            },
            JsComputedMemberExpression(x) => {
                x.object().map(|it| is_specific_id(&it, object_name)).unwrap_or(false) &&
                x.member().map(|it| is_static_text(&it, property_name)).unwrap_or(false)
            },
            _ => false
        }
    }
}

pub fn is_static_text<T>(expr: &T, property_name: &str) -> bool
where
    T: AstNode<Language = JsLanguage>,
{
    with_static_text(expr, |t| t == property_name).unwrap_or(false)
}

pub fn as_static_text<T>(expr: &T) -> Option<String>
where
    T: AstNode<Language = JsLanguage>,
{
    with_static_text(expr, |t| t.to_owned())
}

pub fn with_static_text<T, F, R>(expr: &T, f: F) -> Option<R>
where
    T: AstNode<Language = JsLanguage>,
    F: FnOnce(&str) -> R,
{
    match_ast! {
        match (expr.syntax()) {
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
