use crate::prelude::*;
use rome_formatter::write;

use crate::FormatNodeFields;
use rome_js_syntax::JsArrayAssignmentPatternRestElement;
use rome_js_syntax::JsArrayAssignmentPatternRestElementFields;

impl FormatNodeFields<JsArrayAssignmentPatternRestElement>
    for FormatNodeRule<JsArrayAssignmentPatternRestElement>
{
    fn fmt_fields(
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
