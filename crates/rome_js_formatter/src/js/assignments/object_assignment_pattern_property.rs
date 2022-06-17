use crate::prelude::*;
use crate::utils::JsAnyAssignmentLike;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::JsObjectAssignmentPatternProperty;

impl FormatNodeFields<JsObjectAssignmentPatternProperty>
    for FormatNodeRule<JsObjectAssignmentPatternProperty>
{
    fn fmt_fields(
        node: &JsObjectAssignmentPatternProperty,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        write!(f, [JsAnyAssignmentLike::from(node.clone())])
    }
}
