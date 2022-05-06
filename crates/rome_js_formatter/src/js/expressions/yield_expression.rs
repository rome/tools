use crate::format_traits::FormatOptional;
use rome_formatter::FormatResult;

use crate::{formatted, Format, FormatElement, FormatNode, Formatter};

use rome_js_syntax::JsYieldExpression;
use rome_js_syntax::JsYieldExpressionFields;

impl FormatNode for JsYieldExpression {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsYieldExpressionFields {
            yield_token,
            argument,
        } = self.as_fields();

        let argument = argument.format_or_empty(formatter)?;

        formatted![formatter, yield_token.format(formatter)?, argument]
    }
}
