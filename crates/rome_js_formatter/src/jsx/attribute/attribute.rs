use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::{JsxAttribute, JsxAttributeFields};

impl FormatNodeFields<JsxAttribute> for FormatNodeRule<JsxAttribute> {
    fn format_fields(
        node: &JsxAttribute,
        formatter: &Formatter<JsFormatContext>,
    ) -> FormatResult<FormatElement> {
        let JsxAttributeFields { name, initializer } = node.as_fields();

        formatted![formatter, [name.format(), initializer.format()]]
    }
}
