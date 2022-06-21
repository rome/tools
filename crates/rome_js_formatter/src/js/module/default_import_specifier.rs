use crate::prelude::*;
use rome_formatter::write;

use rome_js_syntax::JsDefaultImportSpecifier;
use rome_js_syntax::JsDefaultImportSpecifierFields;

#[derive(Debug, Clone, Default)]
pub struct FormatJsDefaultImportSpecifier;

impl FormatNodeRule<JsDefaultImportSpecifier> for FormatJsDefaultImportSpecifier {
    fn fmt_fields(&self, node: &JsDefaultImportSpecifier, f: &mut JsFormatter) -> FormatResult<()> {
        let JsDefaultImportSpecifierFields {
            local_name,
            trailing_comma_token,
        } = node.as_fields();

        write![f, [local_name.format(), trailing_comma_token.format()]]
    }
}
