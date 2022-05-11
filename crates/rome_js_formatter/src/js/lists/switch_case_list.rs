use crate::generated::FormatJsSwitchCaseList;
use crate::prelude::*;
use rome_js_syntax::JsSwitchCaseList;

impl FormatRule<JsSwitchCaseList> for FormatJsSwitchCaseList {
    fn format(node: &JsSwitchCaseList, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(formatter.format_list(node))
    }
}
