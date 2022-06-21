use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::{format_args, write};
use rome_js_syntax::{JsStaticMemberExpression, JsStaticMemberExpressionFields};

impl FormatNodeFields<JsStaticMemberExpression> for FormatNodeRule<JsStaticMemberExpression> {
    fn fmt_fields(node: &JsStaticMemberExpression, f: &mut JsFormatter) -> FormatResult<()> {
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
