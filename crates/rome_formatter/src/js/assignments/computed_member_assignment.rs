use crate::formatter_traits::FormatTokenAndNode;

use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};

use rslint_parser::ast::JsComputedMemberAssignment;

impl ToFormatElement for JsComputedMemberAssignment {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(format_elements![
            self.object().format(formatter)?,
            self.l_brack_token().format(formatter)?,
            self.member().format(formatter)?,
            self.r_brack_token().format(formatter)?,
        ])
    }
}
