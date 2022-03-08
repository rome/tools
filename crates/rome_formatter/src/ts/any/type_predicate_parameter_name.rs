//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::TsAnyTypePredicateParameterName;
impl ToFormatElement for TsAnyTypePredicateParameterName {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsReferenceIdentifier(node) => node.to_format_element(formatter),
            Self::TsThisType(node) => node.to_format_element(formatter),
        }
    }
}
