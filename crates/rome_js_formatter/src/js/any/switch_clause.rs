//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsAnySwitchClause;
use crate::prelude::*;
use rome_js_syntax::JsAnySwitchClause;
impl FormatRule<JsAnySwitchClause> for FormatJsAnySwitchClause {
    type Context = JsFormatContext;
    fn format(node: &JsAnySwitchClause, f: &mut Formatter<Self::Context>) -> FormatResult<()> {
        match node {
            JsAnySwitchClause::JsCaseClause(node) => node.format().format(f),
            JsAnySwitchClause::JsDefaultClause(node) => node.format().format(f),
        }
    }
}
