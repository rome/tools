//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::prelude::*;
use rome_js_syntax::JsAnyForInOrOfInitializer;
impl Format for JsAnyForInOrOfInitializer {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsAnyAssignmentPattern(node) => node.format(formatter),
            Self::JsForVariableDeclaration(node) => node.format(formatter),
        }
    }
}
