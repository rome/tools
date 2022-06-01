use crate::formatter::{FormatSeparatedOptions, TrailingSeparator};
use crate::generated::FormatTsTypeArgumentList;
use crate::prelude::*;
use rome_js_syntax::TsTypeArgumentList;

impl FormatRule<TsTypeArgumentList> for FormatTsTypeArgumentList {
    type Context = JsFormatContext;

    fn format(node: &TsTypeArgumentList, formatter: &JsFormatter) -> FormatResult<FormatElement> {
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
