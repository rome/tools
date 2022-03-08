use crate::formatter_traits::FormatTokenAndNode;

use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};

use rome_js_syntax::JsComputedMemberName;
use rome_js_syntax::JsComputedMemberNameFields;

impl ToFormatElement for JsComputedMemberName {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsComputedMemberNameFields {
            l_brack_token,
            expression,
            r_brack_token,
        } = self.as_fields();

        Ok(format_elements![
            l_brack_token.format(formatter)?,
            expression.format(formatter)?,
            r_brack_token.format(formatter)?,
        ])
    }
}
