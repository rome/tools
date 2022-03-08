use crate::formatter_traits::FormatTokenAndNode;
use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::TsQualifiedName;
use rome_js_syntax::TsQualifiedNameFields;

impl ToFormatElement for TsQualifiedName {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsQualifiedNameFields {
            left,
            dot_token,
            right,
        } = self.as_fields();

        Ok(format_elements![
            left.format(formatter)?,
            dot_token.format(formatter)?,
            right.format(formatter)?,
        ])
    }
}
