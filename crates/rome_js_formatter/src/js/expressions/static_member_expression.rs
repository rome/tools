use crate::prelude::*;

use rome_formatter::{format_args, write};
use rome_js_syntax::{JsStaticMemberExpression, JsStaticMemberExpressionFields};

#[derive(Debug, Clone, Default)]
pub struct FormatJsStaticMemberExpression;

impl FormatNodeRule<JsStaticMemberExpression> for FormatJsStaticMemberExpression {
    fn fmt_fields(&self, node: &JsStaticMemberExpression, f: &mut JsFormatter) -> FormatResult<()> {
        let JsStaticMemberExpressionFields {
            object,
            operator_token,
            member,
        } = node.as_fields();

        write!(
            f,
            [
                object.format(),
                group_elements(&format_args![operator_token.format(), member.format()])
            ]
        )
    }
}
