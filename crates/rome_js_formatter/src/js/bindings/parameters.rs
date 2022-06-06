use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::JsParameters;
use rome_js_syntax::JsParametersFields;

impl FormatNodeFields<JsParameters> for FormatNodeRule<JsParameters> {
    fn fmt_fields(node: &JsParameters, f: &mut JsFormatter) -> FormatResult<()> {
        let JsParametersFields {
            l_paren_token,
            items,
            r_paren_token,
        } = node.as_fields();

        write!(
            f,
            [
                format_delimited(&l_paren_token?, &items.format(), &r_paren_token?,)
                    .soft_block_indent()
            ]
        )
    }
}
