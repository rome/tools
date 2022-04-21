//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{Format, FormatElement, FormatResult, Formatter};
use rome_js_syntax::JsAnyFunctionBody;
impl Format for JsAnyFunctionBody {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsAnyExpression(node) => node.format(formatter),
            Self::JsFunctionBody(node) => node.format(formatter),
        }
    }
}
