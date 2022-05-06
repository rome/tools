use crate::{Format, FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;
use rome_js_syntax::TsIndexSignatureParameter;

impl FormatNode for TsIndexSignatureParameter {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let binding = self.binding().format(formatter)?;
        let type_annotation = self.type_annotation().format(formatter)?;

        formatted![formatter, binding, type_annotation]
    }
}
