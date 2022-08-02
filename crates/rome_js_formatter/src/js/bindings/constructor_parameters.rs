use crate::prelude::*;

use rome_js_syntax::JsConstructorParameters;
use rome_js_syntax::JsConstructorParametersFields;

#[derive(Debug, Clone, Default)]
pub struct FormatJsConstructorParameters;

impl FormatNodeRule<JsConstructorParameters> for FormatJsConstructorParameters {
    fn fmt_fields(&self, node: &JsConstructorParameters, f: &mut JsFormatter) -> FormatResult<()> {
        let JsConstructorParametersFields {
            l_paren_token,
            parameters,
            r_paren_token,
        } = node.as_fields();

        format_delimited(&l_paren_token?, &parameters.format(), &r_paren_token?)
            .soft_block_indent()
            .ungrouped()
            .fmt(f)
    }
}
