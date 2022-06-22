use crate::prelude::*;
use crate::utils::JsAnyAssignmentLike;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::JsObjectAssignmentPatternProperty;

#[derive(Debug, Clone, Default)]
pub struct FormatJsObjectAssignmentPatternProperty;

impl FormatNodeRule<JsObjectAssignmentPatternProperty> for FormatJsObjectAssignmentPatternProperty {
    fn fmt_fields(
        &self,
        node: &JsObjectAssignmentPatternProperty,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        write!(f, [JsAnyAssignmentLike::from(node.clone())])
    }
}
