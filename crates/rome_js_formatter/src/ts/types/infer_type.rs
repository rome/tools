use crate::{
    format_elements, space_token, Format, FormatElement, FormatNode, FormatResult, Formatter,
};
use rome_js_syntax::TsInferType;

impl FormatNode for TsInferType {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let infer = self.infer_token().format(formatter)?;
        let type_parameter = self.type_parameter().format(formatter)?;
        Ok(format_elements![infer, space_token(), type_parameter])
    }
}
