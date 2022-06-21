use crate::prelude::*;

use rome_js_syntax::JsArrayHole;

#[derive(Debug, Clone, Default)]
pub struct FormatJsArrayHole;

impl FormatNodeRule<JsArrayHole> for FormatJsArrayHole {
    fn fmt_fields(&self, _: &JsArrayHole, _: &mut JsFormatter) -> FormatResult<()> {
        Ok(())
    }
}
