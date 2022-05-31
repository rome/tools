use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::{JsxFragment, JsxFragmentFields};

impl FormatNodeFields<JsxFragment> for FormatNodeRule<JsxFragment> {
    fn format_fields(node: &JsxFragment, formatter: &JsFormatter) -> FormatResult<FormatElement> {
        let JsxFragmentFields {
            opening_fragment,
            children,
            closing_fragment,
        } = node.as_fields();

        formatted![
            formatter,
            [
                opening_fragment.format(),
                children.format(),
                closing_fragment.format()
            ]
        ]
    }
}
