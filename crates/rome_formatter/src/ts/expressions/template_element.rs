use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::JsTemplateElement;

impl ToFormatElement for JsTemplateElement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let dollar_curly = formatter.format_token(&self.dollar_curly_token()?)?;
        let expression = formatter.format_node(&self.expression()?)?;
        let r_curly = formatter.format_token(&self.r_curly_token()?)?;
        Ok(format_elements![dollar_curly, expression, r_curly])
    }
}
