use crate::prelude::*;

use crate::parentheses::{ExpressionNode, NeedsParentheses};
use rome_formatter::write;
use rome_js_syntax::{JsAnyExpression, NewTargetFields};
use rome_js_syntax::{JsSyntaxNode, NewTarget};

#[derive(Debug, Clone, Default)]
pub struct FormatNewTarget;

impl FormatNodeRule<NewTarget> for FormatNewTarget {
    fn fmt_fields(&self, node: &NewTarget, f: &mut JsFormatter) -> FormatResult<()> {
        let NewTargetFields {
            new_token,
            dot_token,
            target_token,
        } = node.as_fields();

        write![
            f,
            [
                new_token.format(),
                dot_token.format(),
                target_token.format(),
            ]
        ]
    }

    fn needs_parentheses(&self, item: &NewTarget) -> bool {
        item.needs_parentheses()
    }
}

impl NeedsParentheses for NewTarget {
    fn needs_parentheses(&self) -> bool {
        false
    }

    fn needs_parentheses_with_parent(&self, _parent: &JsSyntaxNode) -> bool {
        false
    }
}

impl ExpressionNode for NewTarget {
    #[inline]
    fn resolve(&self) -> JsAnyExpression {
        self.clone().into()
    }

    fn into_resolved(self) -> JsAnyExpression {
        self.into()
    }
}
