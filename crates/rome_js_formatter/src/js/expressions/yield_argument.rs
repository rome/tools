use crate::prelude::*;
use rome_formatter::write;

use rome_js_syntax::JsYieldArgument;
use rome_js_syntax::JsYieldArgumentFields;

#[derive(Debug, Clone, Default)]
pub struct FormatJsYieldArgument;

impl FormatNodeRule<JsYieldArgument> for FormatJsYieldArgument {
    fn fmt_fields(&self, node: &JsYieldArgument, f: &mut JsFormatter) -> FormatResult<()> {
        let JsYieldArgumentFields {
            star_token,
            expression,
        } = node.as_fields();

        write![f, [star_token.format(), space(), expression.format()]]
    }
}
