//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{Format, Formatter};
use rome_formatter::{FormatElement, FormatResult};
use rome_js_syntax::JsAnyName;
impl Format for JsAnyName {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsName(node) => node.format(formatter),
            Self::JsPrivateName(node) => node.format(formatter),
        }
    }
}
