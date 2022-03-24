//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{prelude::*, FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::JsAnyObjectAssignmentPatternMember;
impl ToFormatElement for JsAnyObjectAssignmentPatternMember {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsObjectAssignmentPatternShorthandProperty(node) => node.format(formatter),
            Self::JsObjectAssignmentPatternProperty(node) => node.format(formatter),
            Self::JsObjectAssignmentPatternRest(node) => node.format(formatter),
            Self::JsUnknownAssignment(node) => node.format(formatter),
        }
    }
}
