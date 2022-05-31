use crate::generated::FormatTsTemplateElementList;
use crate::prelude::*;
use rome_js_syntax::TsTemplateElementList;
use rome_rowan::AstNodeList;

impl FormatRule<TsTemplateElementList> for FormatTsTemplateElementList {
    type Context = JsFormatContext;

    fn format(node: &TsTemplateElementList, f: &mut JsFormatter) -> FormatResult<()> {
        let mut join = f.join();

        for item in node {
            join.entry(&group_elements(&item.format()));
        }

        join.finish()
    }
}
