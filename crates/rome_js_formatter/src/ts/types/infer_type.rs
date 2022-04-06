use crate::formatter_traits::FormatTokenAndNode;
use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rome_js_syntax::TsInferType;

impl ToFormatElement for TsInferType {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let infer = self.infer_token().format(formatter)?;
        let type_parameter = self.type_parameter().format(formatter)?;
        Ok(format_elements![infer, space_token(), type_parameter])
    }
}
