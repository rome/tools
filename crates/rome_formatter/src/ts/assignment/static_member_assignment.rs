use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::JsStaticMemberAssignment;

impl ToFormatElement for JsStaticMemberAssignment {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(format_elements![
            formatter.format_node(self.object()?)?,
            formatter.format_token(&self.dot_token()?)?,
            formatter.format_node(self.member()?)?,
        ])
    }
}
