use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};

use crate::{
    format_elements, space_token, token, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rslint_parser::ast::JsNewExpression;
use rslint_parser::ast::JsNewExpressionFields;

impl ToFormatElement for JsNewExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
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
            // TODO handle TsTypeArgs
            space_token(),
            callee.format(formatter)?,
            arguments,
        ])
    }
}
