use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::JsExportNamedSpecifier;
use rome_js_syntax::JsExportNamedSpecifierFields;

impl FormatNodeFields<JsExportNamedSpecifier> for FormatNodeRule<JsExportNamedSpecifier> {
    fn format_fields(
        node: &JsExportNamedSpecifier,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let JsExportNamedSpecifierFields {
            type_token,
            local_name,
            as_token,
            exported_name,
        } = node.as_fields();

        formatted![
            formatter,
            [
                type_token
                    .format()
                    .with_or_empty(|type_token| formatted![formatter, [type_token, space_token()]]),
                local_name.format(),
                space_token(),
                as_token.format(),
                space_token(),
                exported_name.format()
            ]
        ]
    }
}
