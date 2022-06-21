use crate::prelude::*;

use rome_formatter::write;
use rome_js_syntax::JsObjectAssignmentPattern;
use rome_js_syntax::JsObjectAssignmentPatternFields;

#[derive(Debug, Clone, Default)]
pub struct FormatJsObjectAssignmentPattern;

impl FormatNodeRule<JsObjectAssignmentPattern> for FormatJsObjectAssignmentPattern {
    fn fmt_fields(node: &JsObjectAssignmentPattern, f: &mut JsFormatter) -> FormatResult<()> {
        let JsObjectAssignmentPatternFields {
            l_curly_token,
            properties,
            r_curly_token,
        } = node.as_fields();

        write!(
            f,
            [
                format_delimited(&l_curly_token?, &properties.format(), &r_curly_token?,)
                    .soft_block_spaces()
            ]
        )
    }
}
