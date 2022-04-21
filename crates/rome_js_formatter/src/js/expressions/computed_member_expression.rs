use crate::format_traits::FormatOptional;
use rome_formatter::FormatResult;

use crate::{format_elements, Format, FormatElement, FormatNode, Formatter};

use rome_js_syntax::JsComputedMemberExpression;
use rome_js_syntax::JsComputedMemberExpressionFields;

impl FormatNode for JsComputedMemberExpression {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
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
