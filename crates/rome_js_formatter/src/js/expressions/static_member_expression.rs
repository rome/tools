use crate::{
    format_elements, group_elements, Format, FormatElement, FormatNode, FormatResult, Formatter,
};

use rome_js_syntax::JsStaticMemberExpression;
use rome_js_syntax::JsStaticMemberExpressionFields;

impl FormatNode for JsStaticMemberExpression {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsStaticMemberExpressionFields {
            object,
            operator_token,
            member,
        } = self.as_fields();

        Ok(group_elements(format_elements![
            object.format(formatter)?,
            operator_token.format(formatter)?,
            member.format(formatter)?,
        ]))
    }
}
