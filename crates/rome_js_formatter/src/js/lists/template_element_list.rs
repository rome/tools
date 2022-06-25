use crate::prelude::*;
use rome_js_syntax::JsTemplateElementList;

#[derive(Debug, Clone, Default)]
pub struct FormatJsTemplateElementList;

impl FormatRule<JsTemplateElementList> for FormatJsTemplateElementList {
    type Context = JsFormatContext;

    fn fmt(&self, node: &JsTemplateElementList, f: &mut JsFormatter) -> FormatResult<()> {
        let mut join = f.join();

        for element in node {
            join.entry(&element.format());
        }

        join.finish()
    }
}
