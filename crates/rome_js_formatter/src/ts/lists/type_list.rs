use crate::generated::FormatTsTypeList;
use crate::prelude::*;
use rome_js_syntax::TsTypeList;

impl FormatRule<TsTypeList> for FormatTsTypeList {
    type Context = JsFormatContext;

    fn fmt(node: &TsTypeList, f: &mut JsFormatter) -> FormatResult<()> {
        // the grouping will be applied by the parent
        f.join_with(&soft_line_break_or_space())
            .entries(
                node.format_separated(token(",")).with_options(
                    FormatSeparatedOptions::default()
                        .with_trailing_separator(TrailingSeparator::Disallowed),
                ),
            )
            .finish()
    }
}
