use crate::generated::FormatJsTemplateElementList;
use crate::prelude::*;
use rome_js_syntax::JsTemplateElementList;

impl FormatRule<JsTemplateElementList> for FormatJsTemplateElementList {
    type Context = JsFormatContext;

    fn format(
        node: &JsTemplateElementList,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        Ok(concat_elements(
            formatter
                .format_all(node.iter().formatted())?
                .map(group_elements),
        ))
    }
}
