use rslint_parser::ast::JsStaticMemberExpression;

use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};

impl ToFormatElement for JsStaticMemberExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(format_elements![
            formatter.format_node(self.object()?)?,
            formatter.format_token(&self.operator()?)?,
            formatter.format_node(self.member()?)?
        ])
    }
}
