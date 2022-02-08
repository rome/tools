use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};

use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};

use rslint_parser::ast::JsComputedMemberExpression;

impl ToFormatElement for JsComputedMemberExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let optional_chain_token = self.optional_chain_token().format_or_empty(formatter)?;

        Ok(format_elements![
            self.object().format(formatter)?,
            optional_chain_token,
            self.l_brack_token().format(formatter)?,
            self.member().format(formatter)?,
            self.r_brack_token().format(formatter)?,
        ])
    }
}
