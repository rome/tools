use crate::formatter::TryFormatNodeListExtension;
use crate::generated::FormatJsSwitchCaseList;
use crate::prelude::*;
use rome_js_syntax::JsSwitchCaseList;

impl FormatRule<JsSwitchCaseList> for FormatJsSwitchCaseList {
    type Context = JsFormatContext;

    fn format(node: &JsSwitchCaseList, f: &mut JsFormatter) -> FormatResult<()> {
        f.join_with(&hard_line_break())
            .entries(node.try_format_nodes())
            .finish()
    }
}
