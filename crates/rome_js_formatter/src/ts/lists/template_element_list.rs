use crate::prelude::*;
use rome_js_syntax::TsTemplateElementList;

#[derive(Debug, Clone, Default)]
pub struct FormatTsTemplateElementList;

impl FormatRule<TsTemplateElementList> for FormatTsTemplateElementList {
    type Context = JsFormatContext;

    fn fmt(&self, node: &TsTemplateElementList, f: &mut JsFormatter) -> FormatResult<()> {
        let mut join = f.join();

        for item in node {
            join.entry(&group(&item.format()));
        }

        join.finish()
    }
}
