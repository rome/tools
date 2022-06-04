use crate::prelude::*;
use rome_formatter::write;

use crate::FormatNodeFields;
use rome_js_syntax::JsExportNamedSpecifier;
use rome_js_syntax::JsExportNamedSpecifierFields;

impl FormatNodeFields<JsExportNamedSpecifier> for FormatNodeRule<JsExportNamedSpecifier> {
    fn fmt_fields(node: &JsExportNamedSpecifier, f: &mut JsFormatter) -> FormatResult<()> {
        let JsExportNamedSpecifierFields {
            type_token,
            local_name,
            as_token,
            exported_name,
        } = node.as_fields();

        if let Some(type_token) = type_token {
            write!(f, [type_token.format(), space_token()])?;
        }

        write![
            f,
            [
                local_name.format(),
                space_token(),
                as_token.format(),
                space_token(),
                exported_name.format()
            ]
        ]
    }
}
