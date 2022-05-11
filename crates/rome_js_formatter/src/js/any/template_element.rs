//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::prelude::*;
use rome_js_syntax::JsAnyTemplateElement;
impl Format for JsAnyTemplateElement {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsTemplateChunkElement(node) => node.format(formatter),
            Self::JsTemplateElement(node) => node.format(formatter),
        }
    }
}
