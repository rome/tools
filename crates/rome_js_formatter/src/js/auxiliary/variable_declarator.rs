use crate::prelude::*;
use crate::utils::JsAnyAssignmentLike;
use rome_formatter::write;
use rome_js_syntax::JsVariableDeclarator;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsVariableDeclarator;

impl FormatNodeRule<JsVariableDeclarator> for FormatJsVariableDeclarator {
    fn fmt_fields(&self, node: &JsVariableDeclarator, f: &mut JsFormatter) -> FormatResult<()> {
        write![f, [JsAnyAssignmentLike::from(node.clone())]]
    }
}
