use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::{format_args, write};
use rome_js_syntax::{JsAnyFunction, JsFunctionExpression};

impl FormatNodeFields<JsFunctionExpression> for FormatNodeRule<JsFunctionExpression> {
    fn format_fields(node: &JsFunctionExpression, f: &mut JsFormatter) -> FormatResult<()> {
        write![f, [JsAnyFunction::from(node.clone()).format()]]
    }
}
