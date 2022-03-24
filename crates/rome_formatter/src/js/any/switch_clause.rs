//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{prelude::*, FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::JsAnySwitchClause;
impl ToFormatElement for JsAnySwitchClause {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsCaseClause(node) => node.format(formatter),
            Self::JsDefaultClause(node) => node.format(formatter),
        }
    }
}
