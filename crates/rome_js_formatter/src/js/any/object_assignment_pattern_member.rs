//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::prelude::*;
use rome_js_syntax::JsAnyObjectAssignmentPatternMember;
impl Format for JsAnyObjectAssignmentPatternMember {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsObjectAssignmentPatternShorthandProperty(node) => node.format(formatter),
            Self::JsObjectAssignmentPatternProperty(node) => node.format(formatter),
            Self::JsObjectAssignmentPatternRest(node) => node.format(formatter),
            Self::JsUnknownAssignment(node) => node.format(formatter),
        }
    }
}
