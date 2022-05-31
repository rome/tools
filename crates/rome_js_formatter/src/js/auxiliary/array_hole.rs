use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::JsArrayHole;

impl FormatNodeFields<JsArrayHole> for FormatNodeRule<JsArrayHole> {
    fn format_fields(_: &JsArrayHole, _: &mut JsFormatter) -> FormatResult<()> {
        Ok(())
    }
}
