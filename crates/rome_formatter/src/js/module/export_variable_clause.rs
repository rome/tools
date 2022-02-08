use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};

use crate::{format_elements, token, FormatElement, FormatResult, Formatter, ToFormatElement};

use rslint_parser::ast::JsExportVariableClause;

impl ToFormatElement for JsExportVariableClause {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let declarations = self.declarations().format(formatter)?;
        let semicolon = self.semicolon_token().format_or(formatter, || token(";"))?;

        Ok(format_elements![declarations, semicolon])
    }
}
