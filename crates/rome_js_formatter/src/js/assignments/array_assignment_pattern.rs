use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::JsArrayAssignmentPattern;
use rome_js_syntax::JsArrayAssignmentPatternFields;

impl FormatNodeFields<JsArrayAssignmentPattern> for FormatNodeRule<JsArrayAssignmentPattern> {
    fn fmt_fields(node: &JsArrayAssignmentPattern, f: &mut JsFormatter) -> FormatResult<()> {
        let JsArrayAssignmentPatternFields {
            l_brack_token,
            elements,
            r_brack_token,
        } = node.as_fields();

        write!(
            f,
            [
                format_delimited(&l_brack_token?, &elements.format(), &r_brack_token?,)
                    .soft_block_indent()
            ]
        )
    }
}
