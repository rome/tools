use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::JsExportNamedFromSpecifier;
use rome_js_syntax::JsExportNamedFromSpecifierFields;

impl FormatNodeFields<JsExportNamedFromSpecifier> for FormatNodeRule<JsExportNamedFromSpecifier> {
    fn format_fields(
        node: &JsExportNamedFromSpecifier,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let JsExportNamedFromSpecifierFields {
            type_token,
            source_name,
            export_as,
        } = node.as_fields();

        formatted![
            formatter,
            [
                type_token
                    .format()
                    .with_or_empty(|type_token| formatted![formatter, [type_token, space_token()]]),
                source_name.format(),
                export_as
                    .format()
                    .with_or_empty(|export_as| formatted![formatter, [space_token(), export_as]])
            ]
        ]
    }
}
