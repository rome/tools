//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{Format, FormatElement, FormatResult, Formatter};
use rome_js_syntax::JsAnyRoot;
impl Format for JsAnyRoot {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsScript(node) => node.format(formatter),
            Self::JsModule(node) => node.format(formatter),
            Self::JsExpressionSnipped(node) => node.format(formatter),
        }
    }
}
