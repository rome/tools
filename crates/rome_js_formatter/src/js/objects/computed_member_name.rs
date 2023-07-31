use crate::prelude::*;

use rome_formatter::write;
use rome_js_syntax::JsComputedMemberName;
use rome_js_syntax::JsComputedMemberNameFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsComputedMemberName;

impl FormatNodeRule<JsComputedMemberName> for FormatJsComputedMemberName {
    fn fmt_fields(&self, node: &JsComputedMemberName, f: &mut JsFormatter) -> FormatResult<()> {
        let JsComputedMemberNameFields {
            l_brack_token,
            expression,
            r_brack_token,
        } = node.as_fields();

        write![
            f,
            [
                l_brack_token.format(),
                expression.format(),
                r_brack_token.format(),
            ]
        ]
    }
}
