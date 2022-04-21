use crate::{format_elements, Format, FormatElement, FormatNode, FormatResult, Formatter};

use rome_js_syntax::JsComputedMemberName;
use rome_js_syntax::JsComputedMemberNameFields;

impl FormatNode for JsComputedMemberName {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
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
