use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::JsYieldArgument;
use rome_js_syntax::JsYieldArgumentFields;

impl FormatNodeFields<JsYieldArgument> for FormatNodeRule<JsYieldArgument> {
    fn format_fields(
        node: &JsYieldArgument,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let JsYieldArgumentFields {
            star_token,
            expression,
        } = node.as_fields();

        formatted![
            formatter,
            [star_token.format(), space_token(), expression.format()]
        ]
    }
}
