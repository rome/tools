use crate::formatter_traits::FormatTokenAndNode;
use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rome_js_syntax::TsTypeofType;

impl ToFormatElement for TsTypeofType {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let r#typeof = self.typeof_token().format(formatter)?;
        let expression_name = self.expression_name().format(formatter)?;
        Ok(format_elements![r#typeof, space_token(), expression_name])
    }
}
