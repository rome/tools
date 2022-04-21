use crate::{Format, FormatElement, FormatNode, FormatResult, Formatter};

use rome_js_syntax::JsParameters;
use rome_js_syntax::JsParametersFields;

impl FormatNode for JsParameters {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsParametersFields {
            l_paren_token,
            items,
            r_paren_token,
        } = self.as_fields();

        formatter.format_delimited_soft_block_indent(
            &l_paren_token?,
            items.format(formatter)?,
            &r_paren_token?,
        )
    }
}
