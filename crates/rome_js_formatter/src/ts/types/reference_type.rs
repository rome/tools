
use crate::{Format, FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;
use rome_js_syntax::TsReferenceType;

impl FormatNode for TsReferenceType {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let name = self.name().format(formatter)?;
        formatted![formatter, name, self.type_arguments()]
    }
}
