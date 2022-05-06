use crate::format_traits::FormatOptional;
use crate::{Format, FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;
use rome_js_syntax::TsReferenceType;

impl FormatNode for TsReferenceType {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let name = self.name().format(formatter)?;
        let type_arguments = self.type_arguments().format_or_empty(formatter)?;
        formatted![formatter, name, type_arguments]
    }
}
