use crate::formatter::FormatSeparatedExtension;
use crate::generated::FormatJsExportNamedSpecifierList;
use crate::prelude::*;
use rome_js_syntax::JsExportNamedSpecifierList;

impl FormatRule<JsExportNamedSpecifierList> for FormatJsExportNamedSpecifierList {
    type Context = JsFormatContext;

    fn format(node: &JsExportNamedSpecifierList, f: &mut JsFormatter) -> FormatResult<()> {
        f.join_with(soft_line_break_or_space())
            .entries(node.format_separated(token(",")))
            .finish()
    }
}
