use crate::{Format, FormatElement, FormatNode, FormatResult, Formatter};
use rome_js_syntax::TsIdentifierBinding;

impl FormatNode for TsIdentifierBinding {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.name_token().format(formatter)
    }
}
