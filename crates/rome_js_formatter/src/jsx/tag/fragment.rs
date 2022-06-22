use crate::prelude::*;

use rome_formatter::write;
use rome_js_syntax::{JsxFragment, JsxFragmentFields};

#[derive(Debug, Clone, Default)]
pub struct FormatJsxFragment;

impl FormatNodeRule<JsxFragment> for FormatJsxFragment {
    fn fmt_fields(&self, node: &JsxFragment, f: &mut JsFormatter) -> FormatResult<()> {
        let JsxFragmentFields {
            opening_fragment,
            children,
            closing_fragment,
        } = node.as_fields();

        write![
            f,
            [
                opening_fragment.format(),
                children.format(),
                closing_fragment.format()
            ]
        ]
    }
}
