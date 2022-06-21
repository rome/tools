//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::prelude::*;
use rome_js_syntax::JsAnySwitchClause;
#[derive(Debug, Clone, Default)]
pub struct FormatJsAnySwitchClause;
impl FormatRule<JsAnySwitchClause> for FormatJsAnySwitchClause {
    type Context = JsFormatContext;
    fn fmt(node: &JsAnySwitchClause, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsAnySwitchClause::JsCaseClause(node) => node.format().fmt(f),
            JsAnySwitchClause::JsDefaultClause(node) => node.format().fmt(f),
        }
    }
}
