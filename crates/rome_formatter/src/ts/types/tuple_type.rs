use crate::formatter_traits::FormatTokenAndNode;
use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::TsTupleType;

impl ToFormatElement for TsTupleType {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        formatter.format_delimited_soft_block_indent(
            &self.l_brack_token()?,
            self.elements().format(formatter)?,
            &self.r_brack_token()?,
        )
    }
}
