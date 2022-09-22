use crate::parentheses::NeedsParentheses;
use crate::FormatUnknownNodeRule;
use rome_js_syntax::{JsSyntaxNode, JsUnknownExpression};

#[derive(Debug, Clone, Default)]
pub struct FormatJsUnknownExpression;

impl FormatUnknownNodeRule<JsUnknownExpression> for FormatJsUnknownExpression {}

impl NeedsParentheses for JsUnknownExpression {
    #[inline]
    fn needs_parentheses(&self) -> bool {
        false
    }

    #[inline]
    fn needs_parentheses_with_parent(&self, _parent: &JsSyntaxNode) -> bool {
        self.needs_parentheses()
    }
}
