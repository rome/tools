use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::ast::JsLabeledStatement;

impl ToFormatElement for JsLabeledStatement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let label = formatter.format_token(&self.label_token()?)?;
        let colon = formatter.format_token(&self.colon_token()?)?;
        let statement = formatter.format_node(&self.body()?)?;

        Ok(format_elements![label, colon, space_token(), statement])
    }
}
