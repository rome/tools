use crate::formatter_traits::FormatTokenAndNode;
use crate::utils::format_with_semicolon;
use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rome_js_syntax::JsThrowStatement;
use rome_js_syntax::JsThrowStatementFields;

impl ToFormatElement for JsThrowStatement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsThrowStatementFields {
            throw_token,
            argument,
            semicolon_token,
        } = self.as_fields();

        let throw_token = throw_token.format(formatter)?;
        let exception = argument.format(formatter)?;

        format_with_semicolon(
            formatter,
            format_elements![throw_token, space_token(), exception],
            semicolon_token,
        )
    }
}
