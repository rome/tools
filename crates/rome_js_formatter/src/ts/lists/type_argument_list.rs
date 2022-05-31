use crate::formatter::{FormatSeparatedExtension, FormatSeparatedOptions, TrailingSeparator};
use crate::generated::FormatTsTypeArgumentList;
use crate::prelude::*;
use rome_js_syntax::TsTypeArgumentList;

impl FormatRule<TsTypeArgumentList> for FormatTsTypeArgumentList {
    type Context = JsFormatContext;

    fn format(node: &TsTypeArgumentList, f: &mut JsFormatter) -> FormatResult<()> {
        f.join_with(&soft_line_break_or_space())
            .entries(
                node.format_separated_with_options(
                    token(","),
                    FormatSeparatedOptions::default()
                        .with_trailing_separator(TrailingSeparator::Disallowed),
                ),
            )
            .finish()
    }
}
