use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};

use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};

use rslint_parser::ast::JsYieldExpression;

impl ToFormatElement for JsYieldExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let argument = self.argument().format_or_empty(formatter)?;

        Ok(format_elements![
            self.yield_token().format(formatter)?,
            argument
        ])
    }
}
