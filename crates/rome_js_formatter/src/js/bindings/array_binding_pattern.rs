use crate::{Format, FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;

use rome_js_syntax::JsArrayBindingPattern;
use rome_js_syntax::JsArrayBindingPatternFields;

impl FormatNode for JsArrayBindingPattern {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsArrayBindingPatternFields {
            l_brack_token,
            elements,
            r_brack_token,
        } = self.as_fields();

        formatter.format_delimited_soft_block_indent(
            &l_brack_token?,
            elements.format(formatter)?,
            &r_brack_token?,
        )
    }
}
