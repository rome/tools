use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};

use crate::{
    format_elements, space_token, token, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rslint_parser::ast::JsNewExpression;

impl ToFormatElement for JsNewExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let arguments = self
            .arguments()
            .format_or(formatter, || format_elements![token("("), token(")")])?;

        Ok(format_elements![
            self.new_token().format(formatter)?,
            // TODO handle TsTypeArgs
            space_token(),
            self.callee().format(formatter)?,
            arguments,
        ])
    }
}
