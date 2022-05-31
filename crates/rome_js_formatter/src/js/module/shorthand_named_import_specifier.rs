use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::JsShorthandNamedImportSpecifier;
use rome_js_syntax::JsShorthandNamedImportSpecifierFields;

impl FormatNodeFields<JsShorthandNamedImportSpecifier>
    for FormatNodeRule<JsShorthandNamedImportSpecifier>
{
    fn format_fields(
        node: &JsShorthandNamedImportSpecifier,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let JsShorthandNamedImportSpecifierFields {
            type_token,
            local_name,
        } = node.as_fields();

        formatted![
            formatter,
            [
                type_token
                    .format()
                    .with_or_empty(|token| formatted![formatter, [token, space_token()]]),
                local_name.format()
            ]
        ]
    }
}
