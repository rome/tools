use crate::prelude::*;

use crate::parentheses::{ExpressionNode, NeedsParentheses};
use rome_formatter::write;
use rome_js_syntax::{JsAnyExpression, JsNewExpression, JsSyntaxKind};
use rome_js_syntax::{JsNewExpressionFields, JsSyntaxNode};

#[derive(Debug, Clone, Default)]
pub struct FormatJsNewExpression;

impl FormatNodeRule<JsNewExpression> for FormatJsNewExpression {
    fn fmt_fields(&self, node: &JsNewExpression, f: &mut JsFormatter) -> FormatResult<()> {
        let JsNewExpressionFields {
            new_token,
            callee,
            type_arguments,
            arguments,
        } = node.as_fields();

        write![
            f,
            [
                new_token.format(),
                space(),
                callee.format(),
                type_arguments.format(),
            ]
        ]?;

        match arguments {
            Some(arguments) => {
                write!(f, [arguments.format()])
            }
            None => {
                write!(
                    f,
                    [
                        format_inserted(JsSyntaxKind::L_PAREN),
                        format_inserted(JsSyntaxKind::R_PAREN)
                    ]
                )
            }
        }
    }

    fn needs_parentheses(&self, item: &JsNewExpression) -> bool {
        item.needs_parentheses()
    }
}

impl NeedsParentheses for JsNewExpression {
    fn needs_parentheses_with_parent(&self, parent: &JsSyntaxNode) -> bool {
        matches!(parent.kind(), JsSyntaxKind::JS_EXTENDS_CLAUSE)
    }
}

impl ExpressionNode for JsNewExpression {
    #[inline]
    fn resolve(&self) -> JsAnyExpression {
        self.clone().into()
    }

    #[inline]
    fn into_resolved(self) -> JsAnyExpression {
        self.into()
    }
}
