use crate::prelude::*;

use crate::builders::format_delimited;
use rome_formatter::{format_args, write};
use rome_js_syntax::JsCatchDeclaration;
use rome_js_syntax::JsCatchDeclarationFields;

#[derive(Debug, Clone, Default)]
pub struct FormatJsCatchDeclaration;

impl FormatNodeRule<JsCatchDeclaration> for FormatJsCatchDeclaration {
    fn fmt_fields(&self, node: &JsCatchDeclaration, f: &mut JsFormatter) -> FormatResult<()> {
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
