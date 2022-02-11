use crate::{
    format_elements, formatter_traits::FormatTokenAndNode, FormatElement, FormatResult, Formatter,
    ToFormatElement,
};
use rslint_parser::ast::JsExpressionSnipped;

impl ToFormatElement for JsExpressionSnipped {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(format_elements![
            self.expression().format(formatter)?,
            self.eof_token().format(formatter)?,
        ])
    }
}
