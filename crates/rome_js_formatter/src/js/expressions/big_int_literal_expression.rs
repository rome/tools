use rome_formatter::write;
use std::borrow::Cow;

use crate::prelude::*;
use crate::utils::string_utils::ToAsciiLowercaseCow;

use crate::parentheses::{ExpressionNode, NeedsParentheses};
use rome_js_syntax::{JsAnyExpression, JsAnyLiteralExpression, JsBigIntLiteralExpressionFields};
use rome_js_syntax::{JsBigIntLiteralExpression, JsSyntaxNode};

#[derive(Debug, Clone, Default)]
pub struct FormatJsBigIntLiteralExpression;

impl FormatNodeRule<JsBigIntLiteralExpression> for FormatJsBigIntLiteralExpression {
    fn fmt_fields(
        &self,
        node: &JsBigIntLiteralExpression,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        let JsBigIntLiteralExpressionFields { value_token } = node.as_fields();
        let value_token = value_token?;

        let original = value_token.text_trimmed();
        match original.to_ascii_lowercase_cow() {
            Cow::Borrowed(_) => write![f, [value_token.format()]],
            Cow::Owned(lowercase) => {
                write!(
                    f,
                    [format_replaced(
                        &value_token,
                        &dynamic_text(&lowercase, value_token.text_trimmed_range().start())
                    )]
                )
            }
        }
    }

    fn needs_parentheses(&self, item: &JsBigIntLiteralExpression) -> bool {
        item.needs_parentheses()
    }
}

impl ExpressionNode for JsBigIntLiteralExpression {
    #[inline]
    fn resolve(&self) -> JsAnyExpression {
        JsAnyExpression::JsAnyLiteralExpression(JsAnyLiteralExpression::from(self.clone()))
    }

    #[inline]
    fn into_resolved(self) -> JsAnyExpression {
        JsAnyExpression::JsAnyLiteralExpression(JsAnyLiteralExpression::from(self))
    }
}

impl NeedsParentheses for JsBigIntLiteralExpression {
    #[inline(always)]
    fn needs_parentheses(&self) -> bool {
        false
    }

    #[inline(always)]
    fn needs_parentheses_with_parent(&self, _parent: &JsSyntaxNode) -> bool {
        false
    }
}
