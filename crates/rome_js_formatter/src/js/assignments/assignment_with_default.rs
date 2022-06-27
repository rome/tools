use crate::prelude::*;
use rome_formatter::write;

use rome_js_syntax::JsAssignmentWithDefault;
use rome_js_syntax::JsAssignmentWithDefaultFields;

#[derive(Debug, Clone, Default)]
pub struct FormatJsAssignmentWithDefault;

impl FormatNodeRule<JsAssignmentWithDefault> for FormatJsAssignmentWithDefault {
    fn fmt_fields(&self, node: &JsAssignmentWithDefault, f: &mut JsFormatter) -> FormatResult<()> {
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
