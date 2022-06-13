use crate::generated::FormatJsExportNamedFromSpecifierList;
use crate::prelude::*;
use rome_js_syntax::JsExportNamedFromSpecifierList;

impl FormatRule<JsExportNamedFromSpecifierList> for FormatJsExportNamedFromSpecifierList {
    type Context = JsFormatContext;

    fn fmt(node: &JsExportNamedFromSpecifierList, f: &mut JsFormatter) -> FormatResult<()> {
        f.join_with(&soft_line_break_or_space())
            .entries(node.format_separated(token(",")))
            .finish()
    }
}
