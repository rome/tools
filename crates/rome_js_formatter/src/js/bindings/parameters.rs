use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::JsParameters;
use rome_js_syntax::JsParametersFields;

impl FormatNodeFields<JsParameters> for FormatNodeRule<JsParameters> {
    fn format_fields(node: &JsParameters, formatter: &JsFormatter) -> FormatResult<FormatElement> {
        let JsParametersFields {
            l_paren_token,
            items,
            r_paren_token,
        } = node.as_fields();

        formatter
            .delimited(
                &l_paren_token?,
                formatted![formatter, [items.format()]]?,
                &r_paren_token?,
            )
            .soft_block_indent()
            .finish()
    }
}
