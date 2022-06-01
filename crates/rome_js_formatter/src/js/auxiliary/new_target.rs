use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::NewTarget;
use rome_js_syntax::NewTargetFields;

impl FormatNodeFields<NewTarget> for FormatNodeRule<NewTarget> {
    fn format_fields(node: &NewTarget, formatter: &JsFormatter) -> FormatResult<FormatElement> {
        let NewTargetFields {
            new_token,
            dot_token,
            target_token,
        } = node.as_fields();

        formatted![
            formatter,
            [
                new_token.format(),
                dot_token.format(),
                target_token.format(),
            ]
        ]
    }
}
