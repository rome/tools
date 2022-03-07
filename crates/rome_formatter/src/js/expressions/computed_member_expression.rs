use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};

use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};

use rome_js_syntax::JsComputedMemberExpression;
use rome_js_syntax::JsComputedMemberExpressionFields;

impl ToFormatElement for JsComputedMemberExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsComputedMemberExpressionFields {
            object,
            optional_chain_token,
            l_brack_token,
            member,
            r_brack_token,
        } = self.as_fields();

        let optional_chain_token = optional_chain_token.format_or_empty(formatter)?;

        Ok(format_elements![
            object.format(formatter)?,
            optional_chain_token,
            l_brack_token.format(formatter)?,
            member.format(formatter)?,
            r_brack_token.format(formatter)?,
        ])
    }
}
