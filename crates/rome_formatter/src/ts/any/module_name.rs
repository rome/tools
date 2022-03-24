//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{prelude::*, FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::TsAnyModuleName;
impl ToFormatElement for TsAnyModuleName {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::TsIdentifierBinding(node) => node.format(formatter),
            Self::TsQualifiedModuleName(node) => node.format(formatter),
        }
    }
}
