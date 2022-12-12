use crate::prelude::*;
use rome_formatter::utils::node_has_leading_newline;
use rome_formatter::write;
use rome_json_syntax::JsonObjectValue;
use rome_rowan::AstNode;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsonObjectValue;

impl FormatNodeRule<JsonObjectValue> for FormatJsonObjectValue {
    fn fmt_fields(&self, node: &JsonObjectValue, f: &mut JsonFormatter) -> FormatResult<()> {
        write!(f, [node.l_curly_token().format(),])?;

        let should_expand = node_has_leading_newline(node.json_member_list().syntax());

        write!(
            f,
            [group(&soft_space_or_block_indent(
                &node.json_member_list().format()
            ))
            .should_expand(should_expand)]
        )?;

        write!(f, [node.r_curly_token().format()])
    }
}
