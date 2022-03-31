use crate::formatter_traits::FormatTokenAndNode;
use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::TsQualifiedModuleName;
use rome_js_syntax::TsQualifiedModuleNameFields;

impl ToFormatElement for TsQualifiedModuleName {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsQualifiedModuleNameFields {
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
