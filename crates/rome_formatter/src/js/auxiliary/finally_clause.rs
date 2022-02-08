use crate::formatter_traits::FormatTokenAndNode;

use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rslint_parser::ast::JsFinallyClause;

impl ToFormatElement for JsFinallyClause {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(format_elements![
            self.finally_token().format(formatter)?,
            space_token(),
            self.body().format(formatter)?
        ])
    }
}
