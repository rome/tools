//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use rome_js_syntax::JsAnySwitchClause;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsAnySwitchClause;
impl FormatRule<JsAnySwitchClause> for FormatJsAnySwitchClause {
    type Context = JsFormatContext;
    fn fmt(&self, node: &JsAnySwitchClause, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsAnySwitchClause::JsCaseClause(node) => node.format().fmt(f),
            JsAnySwitchClause::JsDefaultClause(node) => node.format().fmt(f),
        }
    }
}
