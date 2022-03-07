//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::TsAnyTemplateElement;
impl ToFormatElement for TsAnyTemplateElement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::TsTemplateChunkElement(node) => node.to_format_element(formatter),
            Self::TsTemplateElement(node) => node.to_format_element(formatter),
        }
    }
}
