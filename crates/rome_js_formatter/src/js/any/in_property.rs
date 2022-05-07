//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::prelude::*;
use rome_js_syntax::JsAnyInProperty;
impl Format for JsAnyInProperty {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsPrivateName(node) => node.format(formatter),
            Self::JsAnyExpression(node) => node.format(formatter),
        }
    }
}
