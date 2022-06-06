use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::{JsAnyClass, JsClassExpression};

impl FormatNodeFields<JsClassExpression> for FormatNodeRule<JsClassExpression> {
    fn fmt_fields(node: &JsClassExpression, f: &mut JsFormatter) -> FormatResult<()> {
        write![f, [JsAnyClass::from(node.clone()).format()]]
    }
}
