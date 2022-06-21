use crate::prelude::*;

use rome_formatter::write;
use rome_js_syntax::JsNamedImportSpecifier;
use rome_js_syntax::JsNamedImportSpecifierFields;

#[derive(Debug, Clone, Default)]
pub struct FormatJsNamedImportSpecifier;

impl FormatNodeRule<JsNamedImportSpecifier> for FormatJsNamedImportSpecifier {
    fn fmt_fields(&self, node: &JsNamedImportSpecifier, f: &mut JsFormatter) -> FormatResult<()> {
        let JsNamedImportSpecifierFields {
            type_token,
            name,
            as_token,
            local_name,
        } = node.as_fields();

        if let Some(type_token) = type_token {
            write!(f, [type_token.format(), space_token()])?;
        }

        write![
            f,
            [
                name.format(),
                soft_line_break_or_space(),
                as_token.format(),
                soft_line_break_or_space(),
                local_name.format()
            ]
        ]
    }
}
