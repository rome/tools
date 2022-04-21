//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{Format, FormatElement, FormatResult, Formatter};
use rome_js_syntax::JsAnySwitchClause;
impl Format for JsAnySwitchClause {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsCaseClause(node) => node.format(formatter),
            Self::JsDefaultClause(node) => node.format(formatter),
        }
    }
}
