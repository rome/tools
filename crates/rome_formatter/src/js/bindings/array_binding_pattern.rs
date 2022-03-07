use crate::{
    formatter_traits::FormatTokenAndNode, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rome_js_syntax::JsArrayBindingPattern;
use rome_js_syntax::JsArrayBindingPatternFields;

impl ToFormatElement for JsArrayBindingPattern {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
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
