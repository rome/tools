use crate::formatter::TrailingSeparator;
use crate::generated::FormatJsNamedImportSpecifierList;
use crate::prelude::*;
use rome_js_syntax::JsNamedImportSpecifierList;

impl FormatRule<JsNamedImportSpecifierList> for FormatJsNamedImportSpecifierList {
    type Options = JsFormatOptions;

    fn format(
        node: &JsNamedImportSpecifierList,
        formatter: &Formatter<JsFormatOptions>,
    ) -> FormatResult<FormatElement> {
        Ok(join_elements(
            soft_line_break_or_space(),
            formatter.format_separated(node, || token(","), TrailingSeparator::default())?,
        ))
    }
}
