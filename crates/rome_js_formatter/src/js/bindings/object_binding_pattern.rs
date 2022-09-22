use crate::prelude::*;
use crate::utils::JsObjectPatternLike;
use rome_formatter::write;
use rome_js_syntax::JsObjectBindingPattern;

#[derive(Debug, Clone, Default)]
pub struct FormatJsObjectBindingPattern;

impl FormatNodeRule<JsObjectBindingPattern> for FormatJsObjectBindingPattern {
    fn fmt_fields(&self, node: &JsObjectBindingPattern, f: &mut JsFormatter) -> FormatResult<()> {
        write!(f, [JsObjectPatternLike::from(node.clone())])
    }

    fn fmt_dangling_comments(
        &self,
        _: &JsObjectBindingPattern,
        _: &mut JsFormatter,
    ) -> FormatResult<()> {
        // Handled in `JsObjectPatternLike`
        Ok(())
    }
}
