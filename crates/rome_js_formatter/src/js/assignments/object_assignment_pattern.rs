use crate::prelude::*;
use crate::utils::JsObjectPatternLike;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::JsObjectAssignmentPattern;

impl FormatNodeFields<JsObjectAssignmentPattern> for FormatNodeRule<JsObjectAssignmentPattern> {
    fn fmt_fields(node: &JsObjectAssignmentPattern, f: &mut JsFormatter) -> FormatResult<()> {
        write!(f, [JsObjectPatternLike::from(node.clone())])
    }
}
