use crate::{Format, FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;
use rome_js_syntax::TsUndefinedType;

impl FormatNode for TsUndefinedType {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.undefined_token().format(formatter)
    }
}
