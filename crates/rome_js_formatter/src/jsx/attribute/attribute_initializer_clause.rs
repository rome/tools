use crate::formatter_traits::FormatTokenAndNode;
use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::{JsxAttributeInitializerClause, JsxAttributeInitializerClauseFields};

impl ToFormatElement for JsxAttributeInitializerClause {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsxAttributeInitializerClauseFields { eq_token, value } = self.as_fields();

        Ok(format_elements![
            eq_token.format(formatter)?,
            value.format(formatter)?
        ])
    }
}
