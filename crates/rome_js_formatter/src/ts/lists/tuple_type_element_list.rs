use crate::context::trailing_comma::FormatTrailingComma;
use crate::prelude::*;
use rome_js_syntax::TsTupleTypeElementList;

#[derive(Debug, Clone, Default)]
pub struct FormatTsTupleTypeElementList;

impl FormatRule<TsTupleTypeElementList> for FormatTsTupleTypeElementList {
    type Context = JsFormatContext;

    fn fmt(&self, node: &TsTupleTypeElementList, f: &mut JsFormatter) -> FormatResult<()> {
        let trailing_separator = FormatTrailingComma::All.trailing_separator(f.options());

        f.join_with(&soft_line_break_or_space())
            .entries(
                node.format_separated(",")
                    .with_trailing_separator(trailing_separator)
                    .nodes_grouped(),
            )
            .finish()
    }
}
