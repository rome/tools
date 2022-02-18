use crate::formatter_traits::FormatTokenAndNode;
use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::TsRestTupleTypeElement;

impl ToFormatElement for TsRestTupleTypeElement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let dotdotdot = self.dotdotdot_token().format(formatter)?;
        let ty = self.ty().format(formatter)?;
        Ok(format_elements![dotdotdot, ty])
    }
}
