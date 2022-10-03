use crate::parentheses::NeedsParentheses;
use crate::FormatUnknownNodeRule;
use rome_js_syntax::{JsSyntaxNode, JsUnknownAssignment};

#[derive(Debug, Clone, Default)]
pub struct FormatJsUnknownAssignment;

impl FormatUnknownNodeRule<JsUnknownAssignment> for FormatJsUnknownAssignment {}

impl NeedsParentheses for JsUnknownAssignment {
    #[inline]
    fn needs_parentheses(&self) -> bool {
        false
    }

    #[inline]
    fn needs_parentheses_with_parent(&self, _: &JsSyntaxNode) -> bool {
        false
    }
}
