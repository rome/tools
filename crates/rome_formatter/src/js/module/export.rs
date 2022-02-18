use crate::formatter_traits::FormatTokenAndNode;

use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rslint_parser::ast::JsExport;
use rslint_parser::ast::JsExportFields;

impl ToFormatElement for JsExport {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsExportFields {
            export_token,
            export_clause,
        } = self.as_fields();

        let export_token = export_token.format(formatter)?;
        let export_clause = export_clause.format(formatter)?;
        Ok(format_elements![export_token, space_token(), export_clause])
    }
}
