use crate::formatter_traits::FormatTokenAndNode;
use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::JsImportCallExpression;

impl ToFormatElement for JsImportCallExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(format_elements![
            self.import_token().format(formatter)?,
            self.arguments().format(formatter)?,
        ])
    }
}
