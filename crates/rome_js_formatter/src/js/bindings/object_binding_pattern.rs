use crate::prelude::*;
use crate::utils::JsObjectPatternLike;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::JsObjectBindingPattern;

impl FormatNodeFields<JsObjectBindingPattern> for FormatNodeRule<JsObjectBindingPattern> {
    fn fmt_fields(node: &JsObjectBindingPattern, f: &mut JsFormatter) -> FormatResult<()> {
        write!(f, [JsObjectPatternLike::from(node.clone())])
    }
}
