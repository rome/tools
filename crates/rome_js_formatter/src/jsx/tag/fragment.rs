use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::{format_args, write};
use rome_js_syntax::{JsxFragment, JsxFragmentFields};

impl FormatNodeFields<JsxFragment> for FormatNodeRule<JsxFragment> {
    fn format_fields(node: &JsxFragment, f: &mut JsFormatter) -> FormatResult<()> {
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
