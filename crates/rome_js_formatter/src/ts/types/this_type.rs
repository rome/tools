use crate::{Format, FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;
use rome_js_syntax::TsThisType;

impl FormatNode for TsThisType {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.this_token().format(formatter)
    }
}
