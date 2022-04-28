use crate::Format;
use crate::{
    format_elements, hard_line_break, indent, space_token, FormatElement, FormatNode, Formatter,
};
use rome_formatter::{empty_element, FormatResult};

use rome_js_syntax::JsDefaultClauseFields;
use rome_js_syntax::{JsDefaultClause, JsSyntaxKind};
use rome_rowan::AstNode;

impl FormatNode for JsDefaultClause {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsDefaultClauseFields {
            default_token,
            colon_token,
            consequent,
        } = self.as_fields();

        let syntax_node = consequent.syntax();
        let is_first_child_block_stmt = match syntax_node.first_child() {
            Some(stmt) => stmt.kind() == JsSyntaxKind::JS_BLOCK_STATEMENT,
            None => false,
        };
        let default = default_token.format(formatter)?;
        let colon = colon_token.format(formatter)?;
        let statements = formatter.format_list(consequent);

        let cons = if statements.is_empty() {
            // Skip inserting an indent block is the consequent is empty to print
            // the trailing comments for the case clause inline if there is no
            // block to push them into
            hard_line_break()
        } else if is_first_child_block_stmt {
            format_elements![space_token(), statements]
        } else {
            // no line break needed after because it is added by the indent in the switch statement
            indent(format_elements![hard_line_break(), statements])
        };
        Ok(format_elements![
            default,
            colon,
            space_token(),
            // no line break needed after because it is added by the indent in the switch statement
            cons
        ])
    }
}
