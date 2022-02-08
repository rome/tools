use crate::formatter_traits::FormatTokenAndNode;

use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};

use rslint_parser::ast::JsObjectBindingPatternRest;

impl ToFormatElement for JsObjectBindingPatternRest {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(format_elements![
            self.dotdotdot_token().format(formatter)?,
            self.binding().format(formatter)?,
        ])
    }
}
