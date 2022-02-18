use crate::formatter_traits::FormatTokenAndNode;

use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};

use rslint_parser::ast::JsTemplateElement;
use rslint_parser::ast::JsTemplateElementFields;

impl ToFormatElement for JsTemplateElement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsTemplateElementFields {
            dollar_curly_token,
            expression,
            r_curly_token,
        } = self.as_fields();

        let dollar_curly = dollar_curly_token.format(formatter)?;
        let expression = expression.format(formatter)?;
        let r_curly = r_curly_token.format(formatter)?;
        Ok(format_elements![dollar_curly, expression, r_curly])
    }
}
