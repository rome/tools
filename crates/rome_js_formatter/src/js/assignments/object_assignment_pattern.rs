use crate::parentheses::{AssignmentNode, NeedsParentheses};
use crate::prelude::*;
use crate::utils::JsObjectPatternLike;
use rome_formatter::write;
use rome_js_syntax::{JsAnyAssignmentPattern, JsObjectAssignmentPattern, JsSyntaxNode};

#[derive(Debug, Clone, Default)]
pub struct FormatJsObjectAssignmentPattern;

impl FormatNodeRule<JsObjectAssignmentPattern> for FormatJsObjectAssignmentPattern {
    fn fmt_fields(
        &self,
        node: &JsObjectAssignmentPattern,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        write!(f, [JsObjectPatternLike::from(node.clone())])
    }

    fn needs_parentheses(&self, item: &JsObjectAssignmentPattern) -> bool {
        item.needs_parentheses()
    }
}

impl AssignmentNode for JsObjectAssignmentPattern {
    #[inline]
    fn resolve(&self) -> JsAnyAssignmentPattern {
        self.clone().into()
    }

    #[inline]
    fn into_resolved(self) -> JsAnyAssignmentPattern {
        self.into()
    }
}

impl NeedsParentheses for JsObjectAssignmentPattern {
    #[inline]
    fn needs_parentheses(&self) -> bool {
        false
    }

    #[inline]
    fn needs_parentheses_with_parent(&self, _: &JsSyntaxNode) -> bool {
        false
    }
}
