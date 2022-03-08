use crate::formatter_traits::FormatTokenAndNode;

use crate::{
    format_elements, space_token, token, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rome_js_syntax::JsLabeledStatement;
use rome_js_syntax::JsLabeledStatementFields;

impl ToFormatElement for JsLabeledStatement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsLabeledStatementFields {
            label_token,
            colon_token,
            body,
        } = self.as_fields();

        let label = label_token.format(formatter)?;
        let colon = colon_token.format(formatter)?;

        let statement = body.format(formatter)?;
        if statement.is_empty() {
            // If the body is an empty statement, force semicolon insertion
            Ok(format_elements![label, colon, token(";")])
        } else {
            Ok(format_elements![label, colon, space_token(), statement])
        }
    }
}
