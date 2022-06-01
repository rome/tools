use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::JsConstructorParameters;
use rome_js_syntax::JsConstructorParametersFields;

impl FormatNodeFields<JsConstructorParameters> for FormatNodeRule<JsConstructorParameters> {
    fn format_fields(node: &JsConstructorParameters, f: &mut JsFormatter) -> FormatResult<()> {
        let JsConstructorParametersFields {
            l_paren_token,
            parameters,
            r_paren_token,
        } = node.as_fields();

        write!(
            f,
            [
                f.delimited(&l_paren_token?, &parameters.format(), &r_paren_token?,)
                    .soft_block_indent()
            ]
        )
    }
}
