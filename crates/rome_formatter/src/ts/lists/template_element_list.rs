use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::TsTemplateElementList;
impl ToFormatElement for TsTemplateElementList {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(formatter.format_list(self.clone()))
    }
}