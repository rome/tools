use crate::formatter_traits::FormatTokenAndNode;
use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::TsObjectType;

impl ToFormatElement for TsObjectType {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        formatter.format_delimited_soft_block_spaces(
            &self.l_curly_token()?,
            self.members().format(formatter)?,
            &self.r_curly_token()?,
        )
    }
}
