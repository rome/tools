use crate::formatter::TrailingSeparator;
use crate::{
    join_elements, soft_line_break_or_space, token, Format, FormatElement, FormatResult, Formatter,
};
use rome_js_syntax::TsTypeList;

impl Format for TsTypeList {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        // the grouping will be applied by the parent
        Ok(join_elements(
            soft_line_break_or_space(),
            formatter.format_separated(self, || token(","), TrailingSeparator::Disallowed)?,
        ))
    }
}
