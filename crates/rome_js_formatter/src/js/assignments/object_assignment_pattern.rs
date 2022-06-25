use crate::prelude::*;
use crate::utils::JsObjectPatternLike;
use rome_formatter::write;
use rome_js_syntax::JsObjectAssignmentPattern;

#[derive(Debug, Clone, Default)]
pub struct FormatJsObjectAssignmentPattern;

impl FormatNodeRule<JsObjectAssignmentPattern> for FormatJsObjectAssignmentPattern {
    fn fmt_fields(
        &self,
        node: &JsObjectAssignmentPattern,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        write!(f, [JsObjectPatternLike::from(node.clone())])
    }
}
