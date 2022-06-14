use crate::prelude::*;
use rome_formatter::write;

use crate::FormatNodeFields;
use rome_js_syntax::JsDefaultImportSpecifier;
use rome_js_syntax::JsDefaultImportSpecifierFields;

impl FormatNodeFields<JsDefaultImportSpecifier> for FormatNodeRule<JsDefaultImportSpecifier> {
    fn fmt_fields(node: &JsDefaultImportSpecifier, f: &mut JsFormatter) -> FormatResult<()> {
        let JsDefaultImportSpecifierFields {
            local_name,
            trailing_comma_token,
        } = node.as_fields();

        write![f, [local_name.format(), trailing_comma_token.format()]]
    }
}
