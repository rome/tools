use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::{JsxAttribute, JsxAttributeFields};

impl FormatNodeFields<JsxAttribute> for FormatNodeRule<JsxAttribute> {
    fn fmt_fields(node: &JsxAttribute, f: &mut JsFormatter) -> FormatResult<()> {
        let JsxAttributeFields { name, initializer } = node.as_fields();

        write![f, [name.format(), initializer.format()]]
    }
}
