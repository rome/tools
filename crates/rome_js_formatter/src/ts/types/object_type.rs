use crate::parentheses::NeedsParentheses;
use crate::prelude::*;
use crate::utils::JsObjectLike;
use rome_formatter::write;
use rome_js_syntax::{JsSyntaxNode, TsObjectType};

#[derive(Debug, Clone, Default)]
pub struct FormatTsObjectType;

impl FormatNodeRule<TsObjectType> for FormatTsObjectType {
    fn fmt_fields(&self, node: &TsObjectType, f: &mut JsFormatter) -> FormatResult<()> {
        write!(f, [JsObjectLike::from(node.clone())])
    }

    fn needs_parentheses(&self, item: &TsObjectType) -> bool {
        item.needs_parentheses()
    }

    fn formats_dangling_comments(&self) -> bool {
        true
    }
}

impl NeedsParentheses for TsObjectType {
    fn needs_parentheses_with_parent(&self, _parent: &JsSyntaxNode) -> bool {
        false
    }
}
