use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::JsConstructorParameters;
use rome_js_syntax::JsConstructorParametersFields;

impl FormatNodeFields<JsConstructorParameters> for FormatNodeRule<JsConstructorParameters> {
    fn format_fields(
        node: &JsConstructorParameters,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let JsConstructorParametersFields {
            l_paren_token,
            parameters,
            r_paren_token,
        } = node.as_fields();

        formatter
            .delimited(
                &l_paren_token?,
                formatted![formatter, [parameters.format()]]?,
                &r_paren_token?,
            )
            .soft_block_indent()
            .finish()
    }
}
