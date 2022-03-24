//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::TsAnyModuleReference;
impl ToFormatElement for TsAnyModuleReference {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::TsAnyName(node) => node.to_format_element(formatter),
            Self::TsExternalModuleReference(node) => node.to_format_element(formatter),
        }
    }
}
