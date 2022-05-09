use crate::formatter::TrailingSeparator;
use crate::generated::FormatJsConstructorParameterList;
use crate::prelude::*;
use rome_js_syntax::JsConstructorParameterList;

impl FormatRule<JsConstructorParameterList> for FormatJsConstructorParameterList {
    fn format(
        node: &JsConstructorParameterList,
        formatter: &Formatter,
    ) -> FormatResult<FormatElement> {
        Ok(join_elements(
            soft_line_break_or_space(),
            formatter.format_separated(node, || token(","), TrailingSeparator::default())?,
        ))
    }
}
