use crate::prelude::*;
use rome_js_syntax::TsReferenceType;

impl FormatNode for TsReferenceType {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let name = self.name().format(formatter)?;
        formatted![formatter, name, self.type_arguments()]
    }
}
