use crate::prelude::*;

use rome_formatter::write;
use rome_js_syntax::{JsxClosingFragment, JsxClosingFragmentFields};

#[derive(Debug, Clone, Default)]
pub struct FormatJsxClosingFragment;

impl FormatNodeRule<JsxClosingFragment> for FormatJsxClosingFragment {
    fn fmt_fields(&self, node: &JsxClosingFragment, f: &mut JsFormatter) -> FormatResult<()> {
        let JsxClosingFragmentFields {
            r_angle_token,
            slash_token,
            l_angle_token,
        } = node.as_fields();

        write![
            f,
            [
                l_angle_token.format(),
                slash_token.format(),
                r_angle_token.format()
            ]
        ]
    }
}
