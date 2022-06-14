use crate::prelude::*;
use rome_formatter::write;

use crate::FormatNodeFields;
use rome_js_syntax::JsObjectAssignmentPatternRest;
use rome_js_syntax::JsObjectAssignmentPatternRestFields;

impl FormatNodeFields<JsObjectAssignmentPatternRest>
    for FormatNodeRule<JsObjectAssignmentPatternRest>
{
    fn fmt_fields(node: &JsObjectAssignmentPatternRest, f: &mut JsFormatter) -> FormatResult<()> {
        let JsObjectAssignmentPatternRestFields {
            dotdotdot_token,
            target,
        } = node.as_fields();

        write!(f, [dotdotdot_token.format(), target.format()])
    }
}
