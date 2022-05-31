use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::JsExportNamedShorthandSpecifier;
use rome_js_syntax::JsExportNamedShorthandSpecifierFields;

impl FormatNodeFields<JsExportNamedShorthandSpecifier>
    for FormatNodeRule<JsExportNamedShorthandSpecifier>
{
    fn format_fields(
        node: &JsExportNamedShorthandSpecifier,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let JsExportNamedShorthandSpecifierFields { type_token, name } = node.as_fields();

        formatted![
            formatter,
            [
                type_token
                    .format()
                    .with_or_empty(|type_token| formatted![formatter, [type_token, space_token()]]),
                name.format()
            ]
        ]
    }
}
