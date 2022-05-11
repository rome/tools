use crate::generated::FormatTsTemplateElementList;
use crate::prelude::*;
use rome_js_syntax::TsTemplateElementList;
use rome_rowan::AstNodeList;

impl FormatRule<TsTemplateElementList> for FormatTsTemplateElementList {
    type Options = JsFormatOptions;

    fn format(
        node: &TsTemplateElementList,
        formatter: &Formatter<JsFormatOptions>,
    ) -> FormatResult<FormatElement> {
        Ok(concat_elements(
            formatter.format_all(node.iter().formatted())?,
        ))
    }
}
