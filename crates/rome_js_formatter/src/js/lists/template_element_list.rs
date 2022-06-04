use crate::generated::FormatJsTemplateElementList;
use crate::prelude::*;
use rome_js_syntax::JsTemplateElementList;

impl FormatRule<JsTemplateElementList> for FormatJsTemplateElementList {
    type Context = JsFormatContext;

    fn fmt(node: &JsTemplateElementList, f: &mut JsFormatter) -> FormatResult<()> {
        let mut join = f.join();

        for element in node {
            join.entry(&group_elements(&element.format()));
        }

        join.finish()
    }
}
