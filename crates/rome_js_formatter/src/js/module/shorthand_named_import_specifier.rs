use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::JsShorthandNamedImportSpecifier;
use rome_js_syntax::JsShorthandNamedImportSpecifierFields;

impl FormatNodeFields<JsShorthandNamedImportSpecifier>
    for FormatNodeRule<JsShorthandNamedImportSpecifier>
{
    fn fmt_fields(node: &JsShorthandNamedImportSpecifier, f: &mut JsFormatter) -> FormatResult<()> {
        let JsShorthandNamedImportSpecifierFields {
            type_token,
            local_name,
        } = node.as_fields();

        if let Some(type_token) = type_token {
            write!(f, [type_token.format(), space_token()])?;
        }

        write![f, [local_name.format()]]
    }
}
