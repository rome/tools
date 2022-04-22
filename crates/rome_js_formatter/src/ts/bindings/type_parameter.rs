use crate::format_traits::FormatOptional;
use crate::{format_elements, space_token, Format, FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;
use rome_js_syntax::TsTypeParameter;

impl FormatNode for TsTypeParameter {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
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
