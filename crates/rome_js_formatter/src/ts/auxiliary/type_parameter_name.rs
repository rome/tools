use crate::{Format, FormatElement, FormatNode, FormatResult, Formatter};
use rome_js_syntax::TsTypeParameterName;

impl FormatNode for TsTypeParameterName {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.ident_token().format(formatter)
    }
}
