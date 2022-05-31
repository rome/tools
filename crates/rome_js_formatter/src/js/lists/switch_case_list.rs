use crate::formatter::FormatNodeExtension;
use crate::generated::FormatJsSwitchCaseList;
use crate::prelude::*;
use rome_js_syntax::JsSwitchCaseList;

impl FormatRule<JsSwitchCaseList> for FormatJsSwitchCaseList {
    type Context = JsFormatContext;

    fn format(node: &JsSwitchCaseList, f: &mut JsFormatter) -> FormatResult<()> {
        let mut join = f.join_nodes_with_hardline();

        for case in node {
            join.entry(case.syntax(), &case.format_or_verbatim());
        }

        join.finish()
    }
}
