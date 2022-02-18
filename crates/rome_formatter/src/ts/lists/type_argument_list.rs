use crate::formatter::TrailingSeparator;
use crate::{
    join_elements, soft_line_break, token, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::ast::TsTypeArgumentList;

impl ToFormatElement for TsTypeArgumentList {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(join_elements(
            soft_line_break(),
            formatter.format_separated(self.clone(), || token(","), TrailingSeparator::Allowed)?,
        ))
    }
}
