use crate::formatter_traits::FormatTokenAndNode;

use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rslint_parser::ast::JsExtendsClause;

impl ToFormatElement for JsExtendsClause {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(format_elements![
            self.extends_token().format(formatter)?,
            space_token(),
            self.super_class().format(formatter)?
        ])
    }
}
