use crate::{Format, FormatElement, FormatNode, FormatResult, Formatter};

use rome_js_syntax::JsConstructorParameters;
use rome_js_syntax::JsConstructorParametersFields;

impl FormatNode for JsConstructorParameters {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsConstructorParametersFields {
            l_paren_token,
            parameters,
            r_paren_token,
        } = self.as_fields();

        formatter.format_delimited_soft_block_indent(
            &l_paren_token?,
            parameters.format(formatter)?,
            &r_paren_token?,
        )
    }
}
