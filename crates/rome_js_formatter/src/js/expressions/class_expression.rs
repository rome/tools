use crate::prelude::*;
use crate::utils::format_class::FormatClass;
use crate::FormatNodeFields;
use rome_js_syntax::JsClassExpression;

impl FormatNodeFields<JsClassExpression> for FormatNodeRule<JsClassExpression> {
    fn fmt_fields(node: &JsClassExpression, f: &mut JsFormatter) -> FormatResult<()> {
        FormatClass::from(&node.clone().into()).fmt(f)
    }
}
