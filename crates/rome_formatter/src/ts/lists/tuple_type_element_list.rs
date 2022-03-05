use crate::formatter::TrailingSeparator;
use crate::{
    group_elements, join_elements, soft_line_break_or_space, token, FormatElement, FormatResult,
    Formatter, ToFormatElement,
};
use rslint_syntax::TsTupleTypeElementList;

impl ToFormatElement for TsTupleTypeElementList {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(group_elements(join_elements(
            soft_line_break_or_space(),
            formatter.format_separated(self, || token(","), TrailingSeparator::default())?,
        )))
    }
}
