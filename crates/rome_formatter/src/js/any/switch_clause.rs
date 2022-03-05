//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_syntax::JsAnySwitchClause;
impl ToFormatElement for JsAnySwitchClause {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsCaseClause(node) => node.to_format_element(formatter),
            Self::JsDefaultClause(node) => node.to_format_element(formatter),
        }
    }
}
