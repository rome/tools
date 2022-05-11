//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::prelude::*;
use rome_js_syntax::JsAnyClassMemberName;
impl Format for JsAnyClassMemberName {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsLiteralMemberName(node) => node.format(formatter),
            Self::JsComputedMemberName(node) => node.format(formatter),
            Self::JsPrivateClassMemberName(node) => node.format(formatter),
        }
    }
}
