use crate::{Format, FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;
use rome_js_syntax::TsObjectType;

impl FormatNode for TsObjectType {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        formatter.format_delimited_soft_block_spaces(
            &self.l_curly_token()?,
            self.members().format(formatter)?,
            &self.r_curly_token()?,
        )
    }
}
