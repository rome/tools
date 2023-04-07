use crate::prelude::*;
use rome_formatter::write;
use rome_js_syntax::{JsImportAttribute, JsImportAttributeFields};
use rome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsImportAttribute;
impl FormatNodeRule<JsImportAttribute> for FormatJsImportAttribute {
    fn fmt_fields(&self, node: &JsImportAttribute, f: &mut JsFormatter) -> FormatResult<()> {
        let JsImportAttributeFields {
            with_token,
            l_curly_token,
            attributes,
            r_curly_token,
        } = node.as_fields();

        write![f, [with_token.format(), space(), l_curly_token.format()]]?;

        if attributes.is_empty() {
            let has_dangling = f.comments().has_dangling_comments(node.syntax());
            write!(
                f,
                [
                    has_dangling.then_some(space()),
                    format_dangling_comments(node.syntax()).with_soft_block_indent(),
                    has_dangling.then_some(space()),
                ]
            )?;
        } else {
            write!(
                f,
                [group(&soft_space_or_block_indent(&attributes.format()))]
            )?;
        }

        write!(f, [r_curly_token.format()])
    }

    fn fmt_dangling_comments(
        &self,
        _: &JsImportAttribute,
        _: &mut JsFormatter,
    ) -> FormatResult<()> {
        // Handled as part of `fmt_fields`
        Ok(())
    }
}
