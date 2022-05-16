use crate::formatter::TrailingSeparator;
use crate::generated::FormatTsTypeList;
use crate::prelude::*;
use rome_js_syntax::TsTypeList;

impl FormatRule<TsTypeList> for FormatTsTypeList {
    type Options = JsFormatOptions;

    fn format(
        node: &TsTypeList,
        formatter: &Formatter<JsFormatOptions>,
    ) -> FormatResult<FormatElement> {
        // the grouping will be applied by the parent
        Ok(join_elements(
            soft_line_break_or_space(),
            formatter.format_separated(node, || token(","), TrailingSeparator::Disallowed)?,
        ))
    }
}
