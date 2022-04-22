use crate::utils::format_initializer_clause;
use crate::{format_elements, Format, FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;
use rome_js_syntax::TsEnumMember;

impl FormatNode for TsEnumMember {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let name = self.name().format(formatter)?;
        let initializer = format_initializer_clause(formatter, self.initializer())?;

        Ok(format_elements![name, initializer])
    }
}
