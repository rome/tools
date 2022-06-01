use crate::formatter::{FormatSeparatedOptions, TrailingSeparator};
use crate::generated::FormatTsTypeList;
use crate::prelude::*;
use rome_js_syntax::TsTypeList;

impl FormatRule<TsTypeList> for FormatTsTypeList {
    type Context = JsFormatContext;

    fn format(node: &TsTypeList, formatter: &JsFormatter) -> FormatResult<FormatElement> {
        // the grouping will be applied by the parent
        Ok(join_elements(
            soft_line_break_or_space(),
            formatter.format_separated_with_options(
                node,
                || token(","),
                FormatSeparatedOptions::default()
                    .with_trailing_separator(TrailingSeparator::Disallowed),
            )?,
        ))
    }
}
