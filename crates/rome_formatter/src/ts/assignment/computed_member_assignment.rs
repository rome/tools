use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::JsComputedMemberAssignment;

impl ToFormatElement for JsComputedMemberAssignment {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(format_elements![
            formatter.format_node(&self.object()?)?,
            formatter.format_token(&self.l_brack_token()?)?,
            formatter.format_node(&self.member()?)?,
            formatter.format_token(&self.r_brack_token()?)?,
        ])
    }
}
