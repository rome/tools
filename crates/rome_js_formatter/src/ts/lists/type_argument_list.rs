use crate::formatter::TrailingSeparator;
use crate::{join_elements, soft_line_break_or_space, token, Format, FormatElement, Formatter};
use rome_formatter::FormatResult;
use rome_js_syntax::TsTypeArgumentList;

impl Format for TsTypeArgumentList {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(join_elements(
            soft_line_break_or_space(),
            formatter.format_separated(self, || token(","), TrailingSeparator::Disallowed)?,
        ))
    }
}
