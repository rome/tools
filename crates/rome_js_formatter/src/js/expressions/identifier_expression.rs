use crate::prelude::*;

use crate::parentheses::NeedsParentheses;
use rome_formatter::write;
use rome_js_syntax::JsIdentifierExpressionFields;
use rome_js_syntax::{JsIdentifierExpression, JsSyntaxNode};

#[derive(Debug, Clone, Default)]
pub struct FormatJsIdentifierExpression;

impl FormatNodeRule<JsIdentifierExpression> for FormatJsIdentifierExpression {
    fn fmt_fields(&self, node: &JsIdentifierExpression, f: &mut JsFormatter) -> FormatResult<()> {
        let JsIdentifierExpressionFields { name } = node.as_fields();

        write![f, [name.format()]]
    }

    fn needs_parentheses(&self, item: &JsIdentifierExpression) -> bool {
        item.needs_parentheses()
    }
}

impl NeedsParentheses for JsIdentifierExpression {
    #[inline(always)]
    fn needs_parentheses(&self) -> bool {
        false
    }
    #[inline(always)]
    fn needs_parentheses_with_parent(&self, _parent: &JsSyntaxNode) -> bool {
        false
    }
}
