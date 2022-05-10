use crate::{space_token, Format, FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;
use rome_js_syntax::TsInferType;

impl FormatNode for TsInferType {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let infer = self.infer_token().format(formatter)?;
        let type_parameter = self.type_parameter().format(formatter)?;
        formatted![formatter, infer, space_token(), type_parameter]
    }
}
