use rslint_parser::ast::JsSequenceExpression;

use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};

impl ToFormatElement for JsSequenceExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(format_elements![
            formatter.format_node(self.left()?)?,
            formatter.format_token(&self.comma_token()?)?,
            space_token(),
            formatter.format_node(self.right()?)?
        ])
    }
}
