use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::JsDefaultImportSpecifier;
use rome_js_syntax::JsDefaultImportSpecifierFields;

impl FormatNodeFields<JsDefaultImportSpecifier> for FormatNodeRule<JsDefaultImportSpecifier> {
    fn format_fields(
        node: &JsDefaultImportSpecifier,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let JsDefaultImportSpecifierFields {
            local_name,
            trailing_comma_token,
        } = node.as_fields();

        formatted![
            formatter,
            [local_name.format(), trailing_comma_token.format()]
        ]
    }
}
