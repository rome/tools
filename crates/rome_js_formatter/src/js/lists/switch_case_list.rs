use crate::prelude::*;
use rome_js_syntax::JsSwitchCaseList;

#[derive(Debug, Clone, Default)]
pub struct FormatJsSwitchCaseList;

impl FormatNodeRule<JsSwitchCaseList> for FormatJsSwitchCaseList {
    fn fmt_fields(&self, node: &JsSwitchCaseList, f: &mut JsFormatter) -> FormatResult<()> {
        let mut join = f.join_nodes_with_hardline();

        for case in node {
            join.entry(case.syntax(), &format_or_verbatim(&case));
        }

        join.finish()
    }
}
