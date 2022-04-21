use crate::{Format, FormatElement, FormatResult, Formatter};
use rome_js_syntax::JsSwitchCaseList;
impl Format for JsSwitchCaseList {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(formatter.format_list(self.clone()))
    }
}
