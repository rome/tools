use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::JsCatchDeclaration;
use rome_js_syntax::JsCatchDeclarationFields;

impl FormatNodeFields<JsCatchDeclaration> for FormatNodeRule<JsCatchDeclaration> {
    fn format_fields(
        node: &JsCatchDeclaration,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let JsCatchDeclarationFields {
            l_paren_token,
            binding,
            r_paren_token,
            type_annotation,
        } = node.as_fields();

        formatter
            .delimited(
                &l_paren_token?,
                formatted![formatter, [binding.format(), type_annotation.format()]]?,
                &r_paren_token?,
            )
            .soft_block_indent()
            .finish()
    }
}
