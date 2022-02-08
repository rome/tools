use crate::formatter_traits::FormatTokenAndNode;

use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rslint_parser::ast::JsExport;

impl ToFormatElement for JsExport {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let export_token = self.export_token().format(formatter)?;
        let export_clause = self.export_clause().format(formatter)?;
        Ok(format_elements![export_token, space_token(), export_clause])
    }
}
