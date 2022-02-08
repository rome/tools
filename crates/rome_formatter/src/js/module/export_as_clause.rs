use crate::formatter_traits::FormatTokenAndNode;

use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rslint_parser::ast::JsExportAsClause;

impl ToFormatElement for JsExportAsClause {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let as_token = self.as_token().format(formatter)?;
        let exported_name = self.exported_name().format(formatter)?;

        Ok(format_elements![as_token, space_token(), exported_name])
    }
}
