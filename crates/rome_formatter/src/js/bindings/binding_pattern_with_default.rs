use crate::formatter_traits::FormatTokenAndNode;
use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::ast::JsBindingPatternWithDefault;

impl ToFormatElement for JsBindingPatternWithDefault {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(format_elements![
            self.pattern().format(formatter)?,
            space_token(),
            self.eq_token().format(formatter)?,
            space_token(),
            self.default().format(formatter)?
        ])
    }
}
