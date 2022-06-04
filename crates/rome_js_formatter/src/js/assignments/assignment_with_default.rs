use crate::prelude::*;
use rome_formatter::write;

use crate::FormatNodeFields;
use rome_js_syntax::JsAssignmentWithDefault;
use rome_js_syntax::JsAssignmentWithDefaultFields;

impl FormatNodeFields<JsAssignmentWithDefault> for FormatNodeRule<JsAssignmentWithDefault> {
    fn fmt_fields(node: &JsAssignmentWithDefault, f: &mut JsFormatter) -> FormatResult<()> {
        let JsAssignmentWithDefaultFields {
            pattern,
            eq_token,
            default,
        } = node.as_fields();

        write!(
            f,
            [
                pattern.format(),
                space_token(),
                eq_token.format(),
                space_token(),
                default.format(),
            ]
        )
    }
}
