use crate::generated::FormatTsTemplateElementList;
use crate::prelude::*;
use rome_js_syntax::TsTemplateElementList;
use rome_rowan::AstNodeList;

impl FormatRule<TsTemplateElementList> for FormatTsTemplateElementList {
    fn format(node: &TsTemplateElementList, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(concat_elements(
            formatter.format_all(node.iter().formatted())?,
        ))
    }
}
