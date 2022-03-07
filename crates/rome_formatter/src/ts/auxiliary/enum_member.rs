use crate::formatter_traits::FormatTokenAndNode;
use crate::utils::format_initializer_clause;
use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::TsEnumMember;

impl ToFormatElement for TsEnumMember {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let name = self.name().format(formatter)?;
        let initializer = format_initializer_clause(formatter, self.initializer())?;

        Ok(format_elements![name, initializer])
    }
}
