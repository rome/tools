use crate::prelude::*;
use crate::utils::format_initializer_clause;
use rome_js_syntax::TsEnumMember;

impl FormatNode for TsEnumMember {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let name = self.name().format(formatter)?;
        let initializer = format_initializer_clause(formatter, self.initializer())?;

        formatted![formatter, name, initializer]
    }
}
