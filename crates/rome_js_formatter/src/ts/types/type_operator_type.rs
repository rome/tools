use crate::formatter_traits::FormatTokenAndNode;
use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rome_js_syntax::TsTypeOperatorType;

impl ToFormatElement for TsTypeOperatorType {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(format_elements![
            self.operator_token().format_with(formatter, |operator| {
                format_elements![operator, space_token()]
            })?,
            self.ty().format(formatter)?
        ])
    }
}
