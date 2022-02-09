use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};

use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};

use rslint_parser::ast::JsCallExpression;

impl ToFormatElement for JsCallExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let name = self.callee().format(formatter)?;
        let option = self
            .optional_chain_token_token()
            .format_or_empty(formatter)?;
        let arguments = self.arguments().format(formatter)?;

        Ok(format_elements![name, option, arguments])
    }
}
