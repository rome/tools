use crate::{Format, FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;
use rome_js_syntax::TsNumberType;

impl FormatNode for TsNumberType {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.number_token().format(formatter)
    }
}
