use crate::formatter_traits::FormatTokenAndNode;

use crate::{
    format_elements, group_elements, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rome_js_syntax::JsStaticMemberExpression;
use rome_js_syntax::JsStaticMemberExpressionFields;

impl ToFormatElement for JsStaticMemberExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsStaticMemberExpressionFields {
            object,
            operator,
            member,
        } = self.as_fields();

        Ok(group_elements(format_elements![
            object.format(formatter)?,
            operator.format(formatter)?,
            member.format(formatter)?,
        ]))
    }
}
