use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};

use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rslint_parser::ast::JsYieldArgument;
use rslint_parser::ast::JsYieldArgumentFields;

impl ToFormatElement for JsYieldArgument {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsYieldArgumentFields {
            star_token,
            expression,
        } = self.as_fields();

        let star_token = star_token.format_or_empty(formatter)?;

        Ok(format_elements![
            star_token,
            space_token(),
            expression.format(formatter)?
        ])
    }
}
