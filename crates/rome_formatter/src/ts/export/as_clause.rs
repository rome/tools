use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::ast::JsExportAsClause;

impl ToFormatElement for JsExportAsClause {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let as_token = formatter.format_token(&self.as_token()?)?;
        let exported_name = formatter.format_node(self.exported_name()?)?;

        Ok(format_elements![as_token, space_token(), exported_name])
    }
}
