use crate::{format_elements, Format, FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;

use rome_js_syntax::JsDefaultImportSpecifier;
use rome_js_syntax::JsDefaultImportSpecifierFields;

impl FormatNode for JsDefaultImportSpecifier {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsDefaultImportSpecifierFields {
            local_name,
            trailing_comma_token,
        } = self.as_fields();

        Ok(format_elements![
            local_name.format(formatter)?,
            trailing_comma_token.format(formatter)?
        ])
    }
}
