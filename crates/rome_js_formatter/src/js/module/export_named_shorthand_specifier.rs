use crate::prelude::*;
use rome_formatter::write;

use rome_js_syntax::JsExportNamedShorthandSpecifier;
use rome_js_syntax::JsExportNamedShorthandSpecifierFields;

#[derive(Debug, Clone, Default)]
pub struct FormatJsExportNamedShorthandSpecifier;

impl FormatNodeRule<JsExportNamedShorthandSpecifier> for FormatJsExportNamedShorthandSpecifier {
    fn fmt_fields(node: &JsExportNamedShorthandSpecifier, f: &mut JsFormatter) -> FormatResult<()> {
        let JsExportNamedShorthandSpecifierFields { type_token, name } = node.as_fields();

        if let Some(type_token) = type_token {
            write!(f, [type_token.format(), space_token()])?;
        }

        write![f, [name.format()]]
    }
}
