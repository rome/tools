use crate::formatter_traits::FormatTokenAndNode;
use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::JsTemplateElement;

impl ToFormatElement for JsTemplateElement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let dollar_curly = self.dollar_curly_token().format(formatter)?;
        let expression = self.expression().format(formatter)?;
        let r_curly = self.r_curly_token().format(formatter)?;
        Ok(format_elements![dollar_curly, expression, r_curly])
    }
}
