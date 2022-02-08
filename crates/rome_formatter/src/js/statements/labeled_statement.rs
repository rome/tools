use crate::formatter_traits::FormatTokenAndNode;

use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rslint_parser::ast::JsLabeledStatement;

impl ToFormatElement for JsLabeledStatement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let label = self.label_token().format(formatter)?;
        let colon = self.colon_token().format(formatter)?;
        let statement = self.body().format(formatter)?;

        Ok(format_elements![label, colon, space_token(), statement])
    }
}
