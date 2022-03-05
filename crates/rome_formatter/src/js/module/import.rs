use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};

use crate::{
    format_elements, space_token, token, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rslint_syntax::JsImport;
use rslint_syntax::JsImportFields;

impl ToFormatElement for JsImport {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsImportFields {
            import_token,
            import_clause,
            semicolon_token,
        } = self.as_fields();

        let import_token = import_token.format(formatter)?;
        let import_clause = import_clause.format(formatter)?;
        let semicolon = semicolon_token.format_or(formatter, || token(";"))?;

        Ok(format_elements![
            import_token,
            space_token(),
            import_clause,
            semicolon
        ])
    }
}
