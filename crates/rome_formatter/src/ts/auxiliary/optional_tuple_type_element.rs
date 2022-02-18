use crate::formatter_traits::FormatTokenAndNode;
use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::TsOptionalTupleTypeElement;

impl ToFormatElement for TsOptionalTupleTypeElement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let ty = self.ty().format(formatter)?;
        let question_mark = self.question_mark_token().format(formatter)?;
        Ok(format_elements![ty, question_mark])
    }
}
