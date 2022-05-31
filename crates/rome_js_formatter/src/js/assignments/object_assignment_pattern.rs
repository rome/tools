use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::{format_args, write};
use rome_js_syntax::JsObjectAssignmentPattern;
use rome_js_syntax::JsObjectAssignmentPatternFields;

impl FormatNodeFields<JsObjectAssignmentPattern> for FormatNodeRule<JsObjectAssignmentPattern> {
    fn format_fields(node: &JsObjectAssignmentPattern, f: &mut JsFormatter) -> FormatResult<()> {
        let JsObjectAssignmentPatternFields {
            l_curly_token,
            properties,
            r_curly_token,
        } = node.as_fields();

        write!(
            f,
            [
                f.delimited(&l_curly_token?, &properties.format(), &r_curly_token?,)
                    .soft_block_spaces()
            ]
        )
    }
}
