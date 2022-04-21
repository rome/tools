use crate::format_traits::FormatOptional;
use crate::{format_elements, Format, FormatElement, FormatNode, FormatResult, Formatter};
use rome_js_syntax::TsReferenceType;

impl FormatNode for TsReferenceType {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let name = self.name().format(formatter)?;
        let type_arguments = self.type_arguments().format_or_empty(formatter)?;
        Ok(format_elements![name, type_arguments])
    }
}
