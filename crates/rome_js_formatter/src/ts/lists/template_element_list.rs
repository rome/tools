use crate::{concat_elements, Format, FormatElement, Formatter};
use rome_formatter::FormatResult;
use rome_js_syntax::TsTemplateElementList;
use rome_rowan::AstNodeList;

impl Format for TsTemplateElementList {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(concat_elements(formatter.format_all(self.iter())?))
    }
}
