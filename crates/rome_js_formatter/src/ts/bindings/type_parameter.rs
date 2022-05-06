use crate::format_traits::FormatOptional;
use crate::{space_token, Format, FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;
use rome_js_syntax::TsTypeParameter;

impl FormatNode for TsTypeParameter {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let name = self.name().format(formatter)?;
        let constraint = self
            .constraint()
            .format_with_or_empty(formatter, |constraint| {
                formatted![formatter, space_token(), constraint]
            })?;
        let default = self.default().format_with_or_empty(formatter, |default| {
            formatted![formatter, space_token(), default]
        })?;
        formatted![formatter, name, constraint, default]
    }
}
