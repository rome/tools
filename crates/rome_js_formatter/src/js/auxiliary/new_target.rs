use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::NewTarget;
use rome_js_syntax::NewTargetFields;

impl FormatNodeFields<NewTarget> for FormatNodeRule<NewTarget> {
    fn fmt_fields(node: &NewTarget, f: &mut JsFormatter) -> FormatResult<()> {
        let NewTargetFields {
            new_token,
            dot_token,
            target_token,
        } = node.as_fields();

        write![
            f,
            [
                new_token.format(),
                dot_token.format(),
                target_token.format(),
            ]
        ]
    }
}
