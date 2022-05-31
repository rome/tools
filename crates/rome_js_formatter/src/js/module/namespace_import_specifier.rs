use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::JsNamespaceImportSpecifier;
use rome_js_syntax::JsNamespaceImportSpecifierFields;

impl FormatNodeFields<JsNamespaceImportSpecifier> for FormatNodeRule<JsNamespaceImportSpecifier> {
    fn format_fields(
        node: &JsNamespaceImportSpecifier,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let JsNamespaceImportSpecifierFields {
            star_token,
            as_token,
            local_name,
        } = node.as_fields();

        let star = star_token.format();
        let as_token = as_token.format();
        let local_name = local_name.format();

        formatted![
            formatter,
            [star, space_token(), as_token, space_token(), local_name]
        ]
    }
}
