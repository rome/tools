use crate::prelude::*;

use rome_formatter::write;
use rome_js_syntax::{JsxOpeningFragment, JsxOpeningFragmentFields};

#[derive(Debug, Clone, Default)]
pub struct FormatJsxOpeningFragment;

impl FormatNodeRule<JsxOpeningFragment> for FormatJsxOpeningFragment {
    fn fmt_fields(&self, node: &JsxOpeningFragment, f: &mut JsFormatter) -> FormatResult<()> {
        let JsxOpeningFragmentFields {
            r_angle_token,
            l_angle_token,
        } = node.as_fields();

        write![f, [l_angle_token.format(), r_angle_token.format()]]
    }
}
