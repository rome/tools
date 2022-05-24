use crate::generated::FormatJsTemplateElementList;
use crate::prelude::*;
use rome_js_syntax::JsTemplateElementList;

impl FormatRule<JsTemplateElementList> for FormatJsTemplateElementList {
    type Options = JsFormatOptions;

    fn format(
        node: &JsTemplateElementList,
        formatter: &Formatter<JsFormatOptions>,
    ) -> FormatResult<FormatElement> {
        Ok(formatter.format_list(node))
    }
}
