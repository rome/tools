use crate::formatter::TrailingSeparator;
use crate::prelude::*;
use rome_js_syntax::JsNamedImportSpecifierList;

impl Format for JsNamedImportSpecifierList {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(join_elements(
            soft_line_break_or_space(),
            formatter.format_separated(self, || token(","), TrailingSeparator::default())?,
        ))
    }
}
