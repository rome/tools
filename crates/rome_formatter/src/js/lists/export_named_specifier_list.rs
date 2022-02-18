use crate::formatter::TrailingSeparator;
use crate::{
    join_elements, soft_line_break_or_space, token, FormatElement, FormatResult, Formatter,
    ToFormatElement,
};
use rslint_parser::ast::JsExportNamedSpecifierList;

impl ToFormatElement for JsExportNamedSpecifierList {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(join_elements(
            soft_line_break_or_space(),
            formatter.format_separated(self.clone(), || token(","), TrailingSeparator::Allowed)?,
        ))
    }
}
