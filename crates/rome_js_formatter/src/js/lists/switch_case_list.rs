use crate::generated::FormatJsSwitchCaseList;
use crate::prelude::*;
use rome_js_syntax::JsSwitchCaseList;

impl FormatRule<JsSwitchCaseList> for FormatJsSwitchCaseList {
    type Context = JsFormatContext;

    fn format(node: &JsSwitchCaseList, formatter: &JsFormatter) -> FormatResult<FormatElement> {
        Ok(formatter.format_list(node))
    }
}
