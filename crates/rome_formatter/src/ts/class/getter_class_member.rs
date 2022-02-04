use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};
use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::ast::JsGetterClassMember;

impl ToFormatElement for JsGetterClassMember {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(format_elements![
            self.static_token()
                .format_with_or_empty(formatter, |token| format_elements![token, space_token()])?,
            self.get_token().format(formatter)?,
            space_token(),
            self.name().format(formatter)?,
            self.l_paren_token().format(formatter)?,
            self.r_paren_token().format(formatter)?,
            space_token(),
            self.body().format(formatter)?
        ])
    }
}
