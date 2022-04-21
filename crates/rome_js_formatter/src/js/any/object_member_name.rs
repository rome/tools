//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{Format, FormatElement, FormatResult, Formatter};
use rome_js_syntax::JsAnyObjectMemberName;
impl Format for JsAnyObjectMemberName {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsLiteralMemberName(node) => node.format(formatter),
            Self::JsComputedMemberName(node) => node.format(formatter),
        }
    }
}
