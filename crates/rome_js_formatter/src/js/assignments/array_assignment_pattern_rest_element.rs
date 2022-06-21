use crate::prelude::*;
use rome_formatter::write;

use rome_js_syntax::JsArrayAssignmentPatternRestElement;
use rome_js_syntax::JsArrayAssignmentPatternRestElementFields;

#[derive(Debug, Clone, Default)]
pub struct FormatJsArrayAssignmentPatternRestElement;

impl FormatNodeRule<JsArrayAssignmentPatternRestElement>
    for FormatJsArrayAssignmentPatternRestElement
{
    fn fmt_fields(
        &self,
        node: &JsArrayAssignmentPatternRestElement,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        let JsArrayAssignmentPatternRestElementFields {
            dotdotdot_token,
            pattern,
        } = node.as_fields();

        write!(f, [dotdotdot_token.format(), pattern.format()])
    }
}
