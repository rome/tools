use crate::formatter_traits::FormatTokenAndNode;
use crate::{
    format_elements, group_elements, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::ast::JsStaticMemberExpression;

impl ToFormatElement for JsStaticMemberExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(group_elements(format_elements![
            self.object().format(formatter)?,
            self.operator().format(formatter)?,
            self.member().format(formatter)?,
        ]))
    }
}
