use crate::formatter_traits::FormatTokenAndNode;
use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_syntax::TsTemplateElement;
use rslint_syntax::TsTemplateElementFields;

impl ToFormatElement for TsTemplateElement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsTemplateElementFields {
            dollar_curly_token,
            ty,
            r_curly_token,
        } = self.as_fields();

        Ok(format_elements![
            dollar_curly_token.format(formatter)?,
            ty.format(formatter)?,
            r_curly_token.format(formatter)?,
        ])
    }
}
