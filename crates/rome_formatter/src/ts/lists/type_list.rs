use crate::formatter::TrailingSeparator;
use crate::{
    join_elements, soft_line_break_or_space, token, FormatElement, FormatResult, Formatter,
    ToFormatElement,
};
use rome_js_syntax::TsTypeList;

impl ToFormatElement for TsTypeList {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        // the grouping will be applied by the parent
        Ok(join_elements(
            soft_line_break_or_space(),
            formatter.format_separated(self, || token(","), TrailingSeparator::Disallowed)?,
        ))
    }
}
