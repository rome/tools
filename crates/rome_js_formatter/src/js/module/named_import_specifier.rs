use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::JsNamedImportSpecifier;
use rome_js_syntax::JsNamedImportSpecifierFields;

impl FormatNodeFields<JsNamedImportSpecifier> for FormatNodeRule<JsNamedImportSpecifier> {
    fn format_fields(
        node: &JsNamedImportSpecifier,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let JsNamedImportSpecifierFields {
            type_token,
            name,
            as_token,
            local_name,
        } = node.as_fields();

        formatted![
            formatter,
            [
                type_token
                    .format()
                    .with_or_empty(|token| formatted![formatter, [token, space_token()]]),
                name.format(),
                soft_line_break_or_space(),
                as_token.format(),
                soft_line_break_or_space(),
                local_name.format()
            ]
        ]
    }
}
