use crate::{Format, FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;
use rome_js_syntax::TsBooleanType;

impl FormatNode for TsBooleanType {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.boolean_token().format(formatter)
    }
}
