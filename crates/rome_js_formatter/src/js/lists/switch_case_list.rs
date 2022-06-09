use crate::generated::FormatJsSwitchCaseList;
use crate::prelude::*;
use rome_js_syntax::JsSwitchCaseList;

impl FormatRule<JsSwitchCaseList> for FormatJsSwitchCaseList {
    type Context = JsFormatContext;

    fn fmt(node: &JsSwitchCaseList, f: &mut JsFormatter) -> FormatResult<()> {
        let mut join = f.join_nodes_with_hardline();

        for case in node {
            join.entry(case.syntax(), &format_or_verbatim(&case));
        }

        join.finish()
    }
}
