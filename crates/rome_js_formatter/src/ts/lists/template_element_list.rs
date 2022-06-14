use crate::generated::FormatTsTemplateElementList;
use crate::prelude::*;
use rome_js_syntax::TsTemplateElementList;

impl FormatRule<TsTemplateElementList> for FormatTsTemplateElementList {
    type Context = JsFormatContext;

    fn fmt(node: &TsTemplateElementList, f: &mut JsFormatter) -> FormatResult<()> {
        let mut join = f.join();

        for item in node {
            join.entry(&group_elements(&item.format()));
        }

        join.finish()
    }
}
