use crate::{formatted, Format, FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;
use rome_js_syntax::JsExpressionSnipped;
use rome_js_syntax::JsExpressionSnippedFields;

impl FormatNode for JsExpressionSnipped {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsExpressionSnippedFields {
            expression,
            eof_token,
        } = self.as_fields();

        formatted![
            formatter,
            expression.format(formatter)?,
            eof_token.format(formatter)?,
        ]
    }
}
