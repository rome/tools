use crate::{Format, FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;
use rome_js_syntax::TsTypeParameterName;

impl FormatNode for TsTypeParameterName {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.ident_token().format(formatter)
    }
}
