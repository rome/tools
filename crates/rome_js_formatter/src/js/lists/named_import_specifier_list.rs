use crate::prelude::*;
use rome_js_syntax::JsNamedImportSpecifierList;

#[derive(Debug, Clone, Default)]
pub struct FormatJsNamedImportSpecifierList;

impl FormatRule<JsNamedImportSpecifierList> for FormatJsNamedImportSpecifierList {
    type Context = JsFormatContext;

    fn fmt(&self, node: &JsNamedImportSpecifierList, f: &mut JsFormatter) -> FormatResult<()> {
        f.join_with(&soft_line_break_or_space())
            .entries(node.format_separated(","))
            .finish()
    }
}
