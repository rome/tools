use crate::generated::FormatJsNamedImportSpecifierList;
use crate::prelude::*;
use rome_js_syntax::JsNamedImportSpecifierList;

impl FormatRule<JsNamedImportSpecifierList> for FormatJsNamedImportSpecifierList {
    type Context = JsFormatContext;

    fn fmt(node: &JsNamedImportSpecifierList, f: &mut JsFormatter) -> FormatResult<()> {
        f.join_with(&soft_line_break_or_space())
            .entries(node.format_separated(token(",")))
            .finish()
    }
}
