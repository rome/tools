use crate::prelude::*;

use rome_formatter::write;
use rome_js_syntax::NewTarget;
use rome_js_syntax::NewTargetFields;

#[derive(Debug, Clone, Default)]
pub struct FormatNewTarget;

impl FormatNodeRule<NewTarget> for FormatNewTarget {
    fn fmt_fields(&self, node: &NewTarget, f: &mut JsFormatter) -> FormatResult<()> {
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
