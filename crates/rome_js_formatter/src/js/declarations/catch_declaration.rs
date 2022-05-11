use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::JsCatchDeclaration;
use rome_js_syntax::JsCatchDeclarationFields;

impl FormatNodeFields<JsCatchDeclaration> for FormatNodeRule<JsCatchDeclaration> {
    fn format_fields(
        node: &JsCatchDeclaration,
        formatter: &Formatter,
    ) -> FormatResult<FormatElement> {
        let JsCatchDeclarationFields {
            l_paren_token,
            binding,
            r_paren_token,
            type_annotation,
        } = node.as_fields();

        formatter.format_delimited_soft_block_indent(
            &l_paren_token?,
            formatted![formatter, [binding.format(), type_annotation.format()]]?,
            &r_paren_token?,
        )
    }
}
