use crate::prelude::*;

use rome_js_syntax::JsParameters;
use rome_js_syntax::JsParametersFields;

#[derive(Debug, Clone, Default)]
pub struct FormatJsParameters;

impl FormatNodeRule<JsParameters> for FormatJsParameters {
    fn fmt_fields(&self, node: &JsParameters, f: &mut JsFormatter) -> FormatResult<()> {
        let JsParametersFields {
            l_paren_token,
            items,
            r_paren_token,
        } = node.as_fields();

        format_delimited(&l_paren_token?, &items.format(), &r_paren_token?)
            .soft_block_indent()
            .ungrouped()
            .fmt(f)
    }
}
