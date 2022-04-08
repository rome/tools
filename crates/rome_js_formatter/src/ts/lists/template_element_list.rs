use crate::{concat_elements, FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::TsTemplateElementList;
use rome_rowan::AstNodeList;

impl ToFormatElement for TsTemplateElementList {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(concat_elements(formatter.format_nodes(self.iter())?))
    }
}
