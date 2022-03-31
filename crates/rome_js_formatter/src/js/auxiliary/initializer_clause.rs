use crate::formatter_traits::FormatTokenAndNode;

use crate::{
    format_elements, hard_group_elements, space_token, FormatElement, FormatResult, Formatter,
    ToFormatElement,
};

use rome_js_syntax::JsInitializerClause;
use rome_js_syntax::JsInitializerClauseFields;

impl ToFormatElement for JsInitializerClause {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsInitializerClauseFields {
            eq_token,
            expression,
        } = self.as_fields();

        Ok(format_elements![
            hard_group_elements(eq_token.format(formatter)?),
            space_token(),
            expression.format(formatter)?
        ])
    }
}
