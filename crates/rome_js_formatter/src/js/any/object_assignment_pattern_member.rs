//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::JsAnyObjectAssignmentPatternMember;
impl ToFormatElement for JsAnyObjectAssignmentPatternMember {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsObjectAssignmentPatternShorthandProperty(node) => {
                node.to_format_element(formatter)
            }
            Self::JsObjectAssignmentPatternProperty(node) => node.to_format_element(formatter),
            Self::JsObjectAssignmentPatternRest(node) => node.to_format_element(formatter),
            Self::JsUnknownAssignment(node) => node.to_format_element(formatter),
        }
    }
}
