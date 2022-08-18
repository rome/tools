use crate::prelude::*;
use rome_formatter::write;

use crate::parentheses::{ExpressionNode, NeedsParentheses};
use rome_js_syntax::{JsAnyExpression, JsSuperExpressionFields};
use rome_js_syntax::{JsSuperExpression, JsSyntaxNode};

#[derive(Debug, Clone, Default)]
pub struct FormatJsSuperExpression;

impl FormatNodeRule<JsSuperExpression> for FormatJsSuperExpression {
    fn fmt_fields(&self, node: &JsSuperExpression, f: &mut JsFormatter) -> FormatResult<()> {
        let JsSuperExpressionFields { super_token } = node.as_fields();

        write![f, [super_token.format()]]
    }

    fn needs_parentheses(&self, item: &JsSuperExpression) -> bool {
        item.needs_parentheses()
    }
}

impl NeedsParentheses for JsSuperExpression {
    #[inline(always)]
    fn needs_parentheses(&self) -> bool {
        false
    }
    #[inline(always)]
    fn needs_parentheses_with_parent(&self, _parent: &JsSyntaxNode) -> bool {
        false
    }
}

impl ExpressionNode for JsSuperExpression {
    #[inline]
    fn resolve(&self) -> JsAnyExpression {
        self.clone().into()
    }

    #[inline]
    fn into_resolved(self) -> JsAnyExpression {
        self.into()
    }
}
