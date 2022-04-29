use crate::Format;
use crate::{
    format_elements, hard_line_break, indent, space_token, FormatElement, FormatNode, Formatter,
};
use rome_formatter::FormatResult;

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

        let first_child_is_block_stmt = matches!(
            syntax_node.first_child().map(|n| n.kind()),
            Some(JsSyntaxKind::JS_BLOCK_STATEMENT)
        );

        let default = default_token.format(formatter)?;
        let colon = colon_token.format(formatter)?;
        let statements = formatter.format_list(consequent);

        let formatted_cons = if statements.is_empty() {
            hard_line_break()
        } else if first_child_is_block_stmt {
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
            formatted_cons
        ])
    }
}
