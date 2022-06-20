use crate::prelude::*;
use crate::utils::JsAnyAssignmentLike;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::JsVariableDeclarator;

impl FormatNodeFields<JsVariableDeclarator> for FormatNodeRule<JsVariableDeclarator> {
    fn fmt_fields(node: &JsVariableDeclarator, f: &mut JsFormatter) -> FormatResult<()> {
        write![f, [JsAnyAssignmentLike::from(node.clone())]]
    }
}
