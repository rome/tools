use rome_js_syntax::{JsAnyStatement, JsSyntaxKind, JsSyntaxNode};
use rome_rowan::AstNode;

pub(crate) fn needs_parenthesis(node: &JsSyntaxNode) -> bool {
    let parent = node.parent();

    // the checks rely on checking parent, we bail early it doesn't exists
    if parent.is_none() {
        return false;
    }

    // SAFETY: protected from the previous check
    let parent = parent.unwrap();

    let great_parent_kind = parent.parent().map(|p| p.kind());

    // statements don't need parenthesis
    if JsAnyStatement::can_cast(parent.kind()) {
        return false;
    }

    // classes check
    if matches!(parent.kind(), JsSyntaxKind::JS_EXTENDS_CLAUSE)
        && matches!(
            great_parent_kind,
            Some(JsSyntaxKind::JS_CLASS_DECLARATION | JsSyntaxKind::JS_CLASS_EXPRESSION)
        )
        && matches!(
            node.kind(),
            JsSyntaxKind::JS_NEW_EXPRESSION
                | JsSyntaxKind::JS_YIELD_EXPRESSION
                | JsSyntaxKind::JS_OBJECT_EXPRESSION
                | JsSyntaxKind::TS_NON_NULL_ASSERTION_EXPRESSION
        )
    {
        return true;
    }

    return false;
}
