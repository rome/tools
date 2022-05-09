use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::JsConstructorParameters;
use rome_js_syntax::JsConstructorParametersFields;

impl FormatNodeFields<JsConstructorParameters> for FormatNodeRule<JsConstructorParameters> {
    fn format_fields(
        node: &JsConstructorParameters,
        formatter: &Formatter,
    ) -> FormatResult<FormatElement> {
        let JsConstructorParametersFields {
            l_paren_token,
            parameters,
            r_paren_token,
        } = node.as_fields();

        formatter.format_delimited_soft_block_indent(
            &l_paren_token?,
            formatted![formatter, parameters.format()]?,
            &r_paren_token?,
        )
    }
}
