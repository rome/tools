use crate::formatter::TrailingSeparator;
use crate::{
    join_elements, soft_line_break_or_space, token, FormatElement, FormatResult, Formatter,
    ToFormatElement,
};
use rome_js_syntax::{AstSeparatedList, TsTypeParameterList};

impl ToFormatElement for TsTypeParameterList {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        // nodes and formatter are not aware of the source type (TSX vs TS), which means we can't
        // exactly pin point the exact case.
        //
        // This is just an heuristic to avoid removing the trailing comma from a TSX grammar.
        // This means that, if we are in a TS context and we have a trailing comma, the formatter won't remove it.
        // It's an edge case, while waiting for a better solution,
        let trailing_separator = if self.len() == 1 && self.trailing_separator().is_some() {
            TrailingSeparator::Mandatory
        } else {
            TrailingSeparator::default()
        };
        Ok(join_elements(
            soft_line_break_or_space(),
            formatter.format_separated(self, || token(","), trailing_separator)?,
        ))
    }
}
