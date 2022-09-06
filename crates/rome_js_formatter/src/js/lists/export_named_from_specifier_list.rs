use crate::prelude::*;
use rome_js_syntax::JsExportNamedFromSpecifierList;

#[derive(Debug, Clone, Default)]
pub struct FormatJsExportNamedFromSpecifierList;

impl FormatRule<JsExportNamedFromSpecifierList> for FormatJsExportNamedFromSpecifierList {
    type Context = JsFormatContext;

    fn fmt(&self, node: &JsExportNamedFromSpecifierList, f: &mut JsFormatter) -> FormatResult<()> {
        f.join_with(&soft_line_break_or_space())
            .entries(node.format_separated(","))
            .finish()
    }
}
