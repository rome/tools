use crate::{
    formatter_traits::FormatTokenAndNode, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rome_js_syntax::JsParameters;
use rome_js_syntax::JsParametersFields;

impl ToFormatElement for JsParameters {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
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
