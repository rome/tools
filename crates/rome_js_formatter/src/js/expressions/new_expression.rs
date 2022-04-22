use crate::format_traits::FormatOptional;
use rome_formatter::FormatResult;

use crate::{format_elements, space_token, token, Format, FormatElement, FormatNode, Formatter};

use rome_js_syntax::JsNewExpression;
use rome_js_syntax::JsNewExpressionFields;

impl FormatNode for JsNewExpression {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsNewExpressionFields {
            new_token,
            callee,
            type_arguments,
            arguments,
        } = self.as_fields();

        let arguments =
            arguments.format_or(formatter, || format_elements![token("("), token(")")])?;

        Ok(format_elements![
            new_token.format(formatter)?,
            space_token(),
            callee.format(formatter)?,
            type_arguments.format_or_empty(formatter)?,
            arguments,
        ])
    }
}
