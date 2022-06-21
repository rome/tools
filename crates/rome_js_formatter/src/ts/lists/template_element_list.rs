use crate::prelude::*;
use rome_js_syntax::TsTemplateElementList;

#[derive(Debug, Clone, Default)]
pub struct FormatTsTemplateElementList;

impl FormatNodeRule<TsTemplateElementList> for FormatTsTemplateElementList {
    fn fmt_fields(&self, node: &TsTemplateElementList, f: &mut JsFormatter) -> FormatResult<()> {
        let mut join = f.join();

        for item in node {
            join.entry(&group_elements(&item.format()));
        }

        join.finish()
    }
}
