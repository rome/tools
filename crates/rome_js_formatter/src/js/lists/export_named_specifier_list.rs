use crate::prelude::*;
use rome_js_syntax::JsExportNamedSpecifierList;

#[derive(Debug, Clone, Default)]
pub struct FormatJsExportNamedSpecifierList;

impl FormatRule<JsExportNamedSpecifierList> for FormatJsExportNamedSpecifierList {
    type Context = JsFormatContext;

    fn fmt(&self, node: &JsExportNamedSpecifierList, f: &mut JsFormatter) -> FormatResult<()> {
        f.join_with(&soft_line_break_or_space())
            .entries(node.format_separated(","))
            .finish()
    }
}
