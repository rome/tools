use crate::prelude::*;
use rome_formatter::write;

use crate::FormatNodeFields;
use rome_js_syntax::JsExportNamedShorthandSpecifier;
use rome_js_syntax::JsExportNamedShorthandSpecifierFields;

impl FormatNodeFields<JsExportNamedShorthandSpecifier>
    for FormatNodeRule<JsExportNamedShorthandSpecifier>
{
    fn fmt_fields(node: &JsExportNamedShorthandSpecifier, f: &mut JsFormatter) -> FormatResult<()> {
        let JsExportNamedShorthandSpecifierFields { type_token, name } = node.as_fields();

        if let Some(type_token) = type_token {
            write!(f, [type_token.format(), space_token()])?;
        }

        write![f, [name.format()]]
    }
}
