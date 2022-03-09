use crate::formatter_traits::FormatTokenAndNode;

use crate::{
    format_elements, space_token, token, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rome_js_syntax::JsLabeledStatementFields;
use rome_js_syntax::{JsAnyStatement, JsLabeledStatement};

impl ToFormatElement for JsLabeledStatement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsLabeledStatementFields {
            label_token,
            colon_token,
            body,
        } = self.as_fields();

        let label = label_token.format(formatter)?;
        let colon = colon_token.format(formatter)?;

        let body = body?;
        if matches!(body, JsAnyStatement::JsEmptyStatement(_)) {
            // If the body is an empty statement, force semicolon insertion
            let statement = body.format(formatter)?;
            Ok(format_elements![label, colon, statement, token(";")])
        } else {
            let statement = body.format(formatter)?;
            Ok(format_elements![label, colon, space_token(), statement])
        }
    }
}
