use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::{format_args, write};
use rome_js_syntax::JsCatchDeclaration;
use rome_js_syntax::JsCatchDeclarationFields;

impl FormatNodeFields<JsCatchDeclaration> for FormatNodeRule<JsCatchDeclaration> {
    fn fmt_fields(node: &JsCatchDeclaration, f: &mut JsFormatter) -> FormatResult<()> {
        let JsCatchDeclarationFields {
            l_paren_token,
            binding,
            r_paren_token,
            type_annotation,
        } = node.as_fields();

        write!(
            f,
            [format_delimited(
                &l_paren_token?,
                &format_args![binding.format(), type_annotation.format()],
                &r_paren_token?,
            )
            .soft_block_indent()]
        )
    }
}
