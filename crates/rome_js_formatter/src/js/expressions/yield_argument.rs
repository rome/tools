use crate::prelude::*;
use rome_formatter::write;

use crate::FormatNodeFields;
use rome_js_syntax::JsYieldArgument;
use rome_js_syntax::JsYieldArgumentFields;

impl FormatNodeFields<JsYieldArgument> for FormatNodeRule<JsYieldArgument> {
    fn fmt_fields(node: &JsYieldArgument, f: &mut JsFormatter) -> FormatResult<()> {
        let JsYieldArgumentFields {
            star_token,
            expression,
        } = node.as_fields();

        write![f, [star_token.format(), space_token(), expression.format()]]
    }
}
