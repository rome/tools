use crate::{
    empty_element, format_elements, hard_line_break, Format, FormatElement, FormatResult, Formatter,
};
use rome_js_syntax::JsDirectiveList;
use rome_rowan::AstNodeList;

impl Format for JsDirectiveList {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        if !self.is_empty() {
            Ok(format_elements![
                formatter.format_list(self.clone()),
                hard_line_break()
            ])
        } else {
            Ok(empty_element())
        }
    }
}
