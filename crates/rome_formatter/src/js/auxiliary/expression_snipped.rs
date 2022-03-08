use crate::{
    format_elements, formatter_traits::FormatTokenAndNode, FormatElement, FormatResult, Formatter,
    ToFormatElement,
};
use rome_js_syntax::JsExpressionSnipped;
use rome_js_syntax::JsExpressionSnippedFields;

impl ToFormatElement for JsExpressionSnipped {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsExpressionSnippedFields {
            expression,
            eof_token,
        } = self.as_fields();

        Ok(format_elements![
            expression.format(formatter)?,
            eof_token.format(formatter)?,
        ])
    }
}
