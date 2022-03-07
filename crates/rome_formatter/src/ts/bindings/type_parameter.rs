use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};
use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rome_js_syntax::TsTypeParameter;

impl ToFormatElement for TsTypeParameter {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let name = self.name().format(formatter)?;
        let constraint = self
            .constraint()
            .format_with_or_empty(formatter, |constraint| {
                format_elements![space_token(), constraint]
            })?;
        let default = self.default().format_with_or_empty(formatter, |default| {
            format_elements![space_token(), default]
        })?;
        Ok(format_elements![name, constraint, default])
    }
}
