use crate::{Format, FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;
use rome_js_syntax::TsNeverType;

impl FormatNode for TsNeverType {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.never_token().format(formatter)
    }
}
