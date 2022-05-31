use crate::prelude::*;
use rome_formatter::write;

use crate::FormatNodeFields;
use rome_js_syntax::JsExportNamedShorthandSpecifier;
use rome_js_syntax::JsExportNamedShorthandSpecifierFields;

impl FormatNodeFields<JsExportNamedShorthandSpecifier>
    for FormatNodeRule<JsExportNamedShorthandSpecifier>
{
    fn format_fields(
        node: &JsExportNamedShorthandSpecifier,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        let JsExportNamedShorthandSpecifierFields { type_token, name } = node.as_fields();

        write![
            f,
            [
                type_token
                    .format()
                    .with_or_empty(|type_token, f| write![f, [type_token, space_token()]]),
                name.format()
            ]
        ]
    }
}
