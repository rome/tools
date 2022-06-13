use crate::generated::FormatJsArrayElementList;
use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::JsArrayExpression;
use rome_js_syntax::JsArrayExpressionFields;

impl FormatNodeFields<JsArrayExpression> for FormatNodeRule<JsArrayExpression> {
    fn fmt_fields(node: &JsArrayExpression, f: &mut JsFormatter) -> FormatResult<()> {
        let JsArrayExpressionFields {
            l_brack_token,
            elements,
            r_brack_token,
        } = node.as_fields();

        let group_id = f.group_id("array");

        let elements = format_with(|f| {
            FormatJsArrayElementList::format_with_group_id(&elements, f, Some(group_id))
        });

        write!(
            f,
            [
                format_delimited(&l_brack_token?, &elements, &r_brack_token?)
                    .soft_block_indent_with_group_id(Some(group_id))
            ]
        )
    }
}
